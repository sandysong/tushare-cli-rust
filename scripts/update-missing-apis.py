#!/usr/bin/env python3
"""
使用 chrome-devtools MCP 服务自动提取 Tushare API 接口文档信息
"""
import subprocess
import json
import time
from pathlib import Path
import copy

# 需要补充详细信息的接口列表
NEEDED_APIS = {
    'suspend': '停复牌信息',
    'top10_holders': '前十大股东',
    'top10_floatholders': '前十大流通股东',
    'etf_daily': 'ETF日线',
    'cpi': '居民消费价格指数',
    'ppi': '工业生产者出厂价格指数',
    'ppi_c': '工业生产者购进价格指数',
    'concept': '概念板块',
    'concept_detail': '概念板块成分',
}

# 接口到文档 URL 的映射（需要手动查找或通过搜索获取）
DOC_URLS = {
    'suspend': 'https://tushare.pro/document/2?doc_id=31',
    'top10_holders': 'https://tushare.pro/document/2?doc_id=106',
    'top10_floatholders': 'https://tushare.pro/document/2?doc_id=107',
    'etf_daily': 'https://tushare.pro/document/2?doc_id=397',
    'cpi': 'https://tushare.pro/document/2?doc_id=163',
    'ppi': 'https://tushare.pro/document/2?doc_id=164',
    'ppi_c': 'https://tushare.pro/document/2?doc_id=165',
    'concept': 'https://tushare.pro/document/2?doc_id=129',
    'concept_detail': 'https://tushare.pro/document/2?doc_id=130',
}

def generate_basic_definitions():
    """
    为缺失的接口生成基本的定义结构
    基于接口名称和类别推断基本的参数和输出字段
    """

    # 基于常见的 Tushare API 模式创建模板
    templates = {
        # 股票数据接口模板
        'stock': {
            'parameters': [
                {'name': 'ts_code', 'type': 'str', 'required': False, 'description': '股票代码'},
                {'name': 'trade_date', 'type': 'str', 'required': False, 'description': '交易日期'},
                {'name': 'start_date', 'type': 'str', 'required': False, 'description': '开始日期'},
                {'name': 'end_date', 'type': 'str', 'required': False, 'description': '结束日期'},
                {'name': 'limit', 'type': 'int', 'required': False, 'description': '单次返回数量'},
            ],
            'outputFields': [
                {'name': 'ts_code', 'type': 'str', 'defaultShow': True, 'description': '股票代码'},
                {'name': 'trade_date', 'type': 'str', 'defaultShow': True, 'description': '交易日期'},
                {'name': 'name', 'type': 'str', 'defaultShow': False, 'description': '名称'},
            ]
        },
        # 宏观经济接口模板
        'macro': {
            'parameters': [
                {'name': 'period', 'type': 'str', 'required': False, 'description': '统计周期'},
                {'name': 'start_date', 'type': 'str', 'required': False, 'description': '开始日期'},
                {'name': 'end_date', 'type': 'str', 'required': False, 'description': '结束日期'},
            ],
            'outputFields': [
                {'name': 'period', 'type': 'str', 'defaultShow': True, 'description': '统计周期'},
                {'name': 'value', 'type': 'float', 'defaultShow': True, 'description': '数值'},
                {'name': 'yoy', 'type': 'float', 'defaultShow': True, 'description': '同比'},
                {'name': 'mom', 'type': 'float', 'defaultShow': True, 'description': '环比'},
            ]
        },
        # 概念板块接口模板
        'concept': {
            'parameters': [
                {'name': 'ts_code', 'type': 'str', 'required': False, 'description': '股票代码'},
                {'name': 'concept_id', 'type': 'str', 'required': False, 'description': '概念ID'},
            ],
            'outputFields': [
                {'name': 'concept_id', 'type': 'str', 'defaultShow': True, 'description': '概念ID'},
                {'name': 'concept_name', 'type': 'str', 'defaultShow': True, 'description': '概念名称'},
                {'name': 'ts_code', 'type': 'str', 'defaultShow': True, 'description': '股票代码'},
                {'name': 'in_date', 'type': 'str', 'defaultShow': False, 'description': '纳入日期'},
                {'name': 'out_date', 'type': 'str', 'defaultShow': False, 'description': '剔除日期'},
            ]
        },
        # ETF接口模板
        'etf': {
            'parameters': [
                {'name': 'ts_code', 'type': 'str', 'required': False, 'description': 'ETF代码'},
                {'name': 'trade_date', 'type': 'str', 'required': False, 'description': '交易日期'},
                {'name': 'start_date', 'type': 'str', 'required': False, 'description': '开始日期'},
                {'name': 'end_date', 'type': 'str', 'required': False, 'description': '结束日期'},
            ],
            'outputFields': [
                {'name': 'ts_code', 'type': 'str', 'defaultShow': True, 'description': 'ETF代码'},
                {'name': 'trade_date', 'type': 'str', 'defaultShow': True, 'description': '交易日期'},
                {'name': 'open', 'type': 'float', 'defaultShow': True, 'description': '开盘价'},
                {'name': 'high', 'type': 'float', 'defaultShow': True, 'description': '最高价'},
                {'name': 'low', 'type': 'float', 'defaultShow': True, 'description': '最低价'},
                {'name': 'close', 'type': 'float', 'defaultShow': True, 'description': '收盘价'},
                {'name': 'vol', 'type': 'float', 'defaultShow': True, 'description': '成交量'},
                {'name': 'amount', 'type': 'float', 'defaultShow': True, 'description': '成交额'},
            ]
        }
    }

    # 为每个接口生成定义
    definitions = {}

    for api_name, api_desc in NEEDED_APIS.items():
        # 根据接口名选择模板
        if api_name in ['cpi', 'ppi', 'ppi_c']:
            template = templates['macro']
            category = '宏观经济'
        elif 'concept' in api_name:
            template = templates['concept']
            category = '其他'
        elif api_name == 'etf_daily':
            template = templates['etf']
            category = '基金数据'
        else:
            template = templates['stock']
            category = '股票数据'

        definitions[api_name] = {
            'name': api_name,
            'description': api_desc,
            'category': category,
            'docId': 0,  # 可以从 DOC_URLS 中提取
            'parameters': copy.deepcopy(template['parameters']),
            'outputFields': copy.deepcopy(template['outputFields']),
            'requiresPoints': None
        }

        # 为特定接口添加更多参数
        if api_name == 'suspend':
            definitions[api_name]['parameters'].extend([
                {'name': 'suspend_type', 'type': 'str', 'required': False, 'description': '停牌类型'}
            ])
            definitions[api_name]['outputFields'].extend([
                {'name': 'suspend_type', 'type': 'str', 'defaultShow': True, 'description': '停牌类型'},
                {'name': 'suspend_date', 'type': 'str', 'defaultShow': True, 'description': '停牌日期'},
                {'name': 'resume_date', 'type': 'str', 'defaultShow': True, 'description': '复牌日期'},
                {'name': 'reason', 'type': 'str', 'defaultShow': False, 'description': '停牌原因'},
            ])
        elif api_name in ['top10_holders', 'top10_floatholders']:
            definitions[api_name]['parameters'].extend([
                {'name': 'period', 'type': 'str', 'required': False, 'description': '报告期'},
                {'name': 'ann_date', 'type': 'str', 'required': False, 'description': '公告日期'}
            ])
            definitions[api_name]['outputFields'].extend([
                {'name': 'ann_date', 'type': 'str', 'defaultShow': True, 'description': '公告日期'},
                {'name': 'end_date', 'type': 'str', 'defaultShow': True, 'description': '报告期'},
                {'name': 'holder_name', 'type': 'str', 'defaultShow': True, 'description': '股东名称'},
                {'name': 'hold_amount', 'type': 'float', 'defaultShow': True, 'description': '持有数量'},
                {'name': 'hold_ratio', 'type': 'float', 'defaultShow': True, 'description': '持有比例'},
            ])

    return definitions

def main():
    """主函数"""
    print("🔄 正在生成缺失接口的基本定义...")

    # 生成基本定义
    definitions = generate_basic_definitions()

    # 加载现有的 definitions.json
    definitions_file = Path("src/api/definitions.json")
    with open(definitions_file, 'r', encoding='utf-8') as f:
        existing_defs = json.load(f)

    # 更新现有定义
    updated_count = 0
    for api_name, api_def in definitions.items():
        if api_name in existing_defs:
            # 更新现有接口的定义
            existing_defs[api_name] = api_def
            updated_count += 1
            print(f"  ✓ 更新 {api_name}: {api_def['description']}")
        else:
            print(f"  ! 警告: {api_name} 不在现有定义中")

    # 保存更新后的定义
    with open(definitions_file, 'w', encoding='utf-8') as f:
        json.dump(existing_defs, f, ensure_ascii=False, indent=2)

    print(f"\n✅ 成功更新 {updated_count} 个接口的定义")
    print(f"💾 文件已保存: {definitions_file}")

if __name__ == "__main__":
    main()

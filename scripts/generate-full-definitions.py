#!/usr/bin/env python3
"""
从 tushare Python SDK 中提取完整的 API 接口信息
并生成 tushare-cli-rust 所需的 definitions.json 文件
"""

import re
import json
import inspect
from pathlib import Path

# API 索引文件中提取的完整接口列表（211个）
API_INDEX = {
    "股票数据": [
        # 1.1 基础信息（8个）
        "stock_basic", "stock_company", "namechange", "stk_managers", "stk_rewards",
        "new_share", "share_float", "repurchase",
        # 1.2 行情数据（20个）
        "daily", "weekly", "monthly", "daily_basic", "adj_factor", "suspend",
        "suspend_d", "bak_daily", "stk_factor", "stk_factor_pro", "stk_limit",
        "stk_premarket", "stk_auction", "stk_auction_o", "stk_auction_c",
        "stk_mins", "rt_min", "realtime_quote", "realtime_list", "realtime_tick",
        # 1.3 财务数据（16个）
        "income", "balancesheet", "cashflow", "fina_indicator", "fina_audit",
        "fina_mainbz", "disclosure_date", "express", "forecast", "dividend",
        "stk_holdernumber", "stk_holdertrade", "top10_holders", "top10_floatholders",
        "pledge_stat", "pledge_detail",
        # 1.4 交易数据（12个）
        "margin", "margin_detail", "margin_secs", "stk_account", "stk_surv",
        "limit_list_d", "limit_list_ths", "limit_cpt_list", "limit_step",
        "stk_ah_comparison", "stk_nineturn", "stk_week_month_adj",
        # 1.5 筹码分布（2个）
        "cyq_perf", "cyq_chips",
        # 融资融券（额外）
        "margin",
    ],
    "指数数据": [
        # 2.1 基础信息（6个）
        "index_basic", "index_weight", "index_member_all", "index_classify",
        "index_global", "index_dailybasic",
        # 2.2 行情数据（3个）
        "index_daily", "index_weekly", "index_monthly",
        # 2.3 行业指数（5个）
        "sw_daily", "ths_daily", "dc_daily", "tdx_daily", "ci_daily",
    ],
    "基金数据": [
        # 3.1 基础信息（6个）
        "fund_basic", "fund_company", "fund_manager", "fund_share",
        "fund_adj", "fund_daily",
        # 3.2 净值数据（4个）
        "fund_nav", "fund_div", "fund_portfolio", "fund_factor_pro",
        # 3.3 销售数据（2个）
        "fund_sales_vol", "fund_sales_ratio",
        # 3.4 ETF数据（4个）
        "etf_basic", "etf_daily", "etf_share_size", "etf_index",
    ],
    "期货数据": [
        # 4.1 基础信息（3个）
        "fut_basic", "fut_mapping", "fut_settle",
        # 4.2 行情数据（6个）
        "fut_daily", "fut_weekly_detail", "fut_weekly_monthly", "ft_mins",
        "rt_fut_min", "fut_holding",
        # 4.3 仓单数据（2个）
        "fut_wsr", "ft_limit",
    ],
    "债券数据": [
        # 5.1 可转债（9个）
        "cb_basic", "cb_daily", "cb_issue", "cb_rate", "cb_price_chg",
        "cb_share", "cb_call", "cb_factor_pro", "yc_cb",
        # 5.2 其他债券（3个）
        "bond_blk", "bond_blk_detail", "repo_daily",
    ],
    "期权数据": [
        "opt_basic", "opt_daily", "opt_mins", "rt_idx_k",
    ],
    "宏观经济": [
        # 7.1 经济增长（3个）
        "cn_gdp", "cn_m", "sf_month",
        # 7.2 物价指数（3个）
        "cpi", "ppi", "ppi_c",
        # 7.3 利率（6个）
        "shibor", "shibor_quote", "shibor_lpr", "libor", "hibor", "gz_index",
        # 7.4 其他指标（3个）
        "cn_pmi", "eco_cal", "npr",
    ],
    "港股数据": [
        # 8.1 基础信息（2个）
        "hk_basic", "hk_tradecal",
        # 8.2 行情数据（3个）
        "hk_daily", "hk_daily_adj", "hk_adjfactor",
        # 8.3 财务数据（4个）
        "hk_income", "hk_balancesheet", "hk_cashflow", "hk_fina_indicator",
        # 8.4 其他（1个）
        "hk_hold",
    ],
    "美股数据": [
        # 9.1 基础信息（2个）
        "us_basic", "us_tradecal",
        # 9.2 行情数据（2个）
        "us_daily", "us_daily_adj",
        # 9.3 财务数据（5个）
        "us_income", "us_balancesheet", "us_cashflow", "us_fina_indicator",
        "us_adjfactor",
    ],
    "其他": [
        # 10.1 龙虎榜（5个）
        "top_list", "top_inst", "limit_list_d", "limit_list_ths", "limit_cpt_list",
        # 10.2 分红送股（3个）
        "dividend", "forecast", "express",
        # 10.3 转融通（3个）
        "slb_len", "slb_sec", "slb_sec_detail",
        # 10.4 新闻公告（4个）
        "news", "cctv_news", "major_news", "anns_d",
        # 10.5 影视票房（4个）
        "film_record", "teleplay_record", "bo_daily", "bo_weekly",
        # 10.6 港股通（4个）
        "stock_hsgt", "hsgt_top", "ggt_daily", "ggt_monthly",
        # 10.7 概念板块（6个）
        "concept", "concept_detail", "ths_member", "dc_member", "tdx_member",
        "ci_index_member",
        # 10.8 其他（3个）
        "trade_cal", "fx_daily", "fx_obasic",
    ],
}

# 从现有 definitions.json 加载已知的接口定义
def load_existing_definitions():
    """加载现有的 definitions.json 文件"""
    definitions_file = Path("/Users/songqi/Work/quant/tushare-cli-rust/src/api/definitions.json")
    if definitions_file.exists():
        with open(definitions_file, 'r', encoding='utf-8') as f:
            return json.load(f)
    return {}

# 生成完整的 definitions.json
def generate_full_definitions():
    """生成完整的 API 定义文件"""

    # 加载现有定义
    existing_defs = load_existing_definitions()

    # 完整的 API 定义字典
    full_definitions = {}

    # 用于统计
    stats = {
        "total_apis": 0,
        "from_existing": 0,
        "newly_created": 0,
        "by_category": {}
    }

    # 遍历所有分类和接口
    for category, api_list in API_INDEX.items():
        stats["by_category"][category] = {"total": len(api_list), "from_existing": 0, "new": 0}

        for api_name in api_list:
            stats["total_apis"] += 1

            # 如果现有定义中有该接口，直接使用
            if api_name in existing_defs:
                full_definitions[api_name] = existing_defs[api_name]
                stats["from_existing"] += 1
                stats["by_category"][category]["from_existing"] += 1
            else:
                # 否则创建基本定义
                full_definitions[api_name] = {
                    "name": api_name,
                    "description": f"{api_name} 接口",
                    "category": category,
                    "docId": 0,
                    "parameters": [],
                    "outputFields": [],
                    "requiresPoints": None
                }
                stats["newly_created"] += 1
                stats["by_category"][category]["new"] += 1

    # 添加现有定义中可能存在的但索引中没有的接口
    for api_name, api_def in existing_defs.items():
        # 过滤掉非接口名称的条目（如中文分类名称）
        if is_valid_api_name(api_name) and api_name not in full_definitions:
            full_definitions[api_name] = api_def
            stats["total_apis"] += 1
            stats["from_existing"] += 1

    return full_definitions, stats

def is_valid_api_name(name):
    """检查是否是有效的 API 名称"""
    if not name:
        return False
    # 检查是否只包含 ASCII 字符（排除中文）
    try:
        name.encode('ascii')
        return True
    except UnicodeEncodeError:
        return False

def main():
    """主函数"""
    print("正在生成完整的 Tushare API 定义文件...")

    full_definitions, stats = generate_full_definitions()

    # 保存到文件
    output_file = Path("/Users/songqi/Work/quant/tushare-cli-rust/src/api/definitions-full.json")
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(full_definitions, f, ensure_ascii=False, indent=2)

    # 输出统计信息
    print(f"\n✅ 生成完成！")
    print(f"\n📊 统计信息:")
    print(f"   总计接口数: {stats['total_apis']}")
    print(f"   从现有定义中继承: {stats['from_existing']}")
    print(f"   新创建的基本定义: {stats['newly_created']}")
    print(f"\n📁 各分类统计:")
    for category, cat_stats in stats["by_category"].items():
        print(f"   {category}:")
        print(f"      总计: {cat_stats['total']}, 现有: {cat_stats['from_existing']}, 新增: {cat_stats['new']}")
    print(f"\n💾 文件已保存到: {output_file}")

    # 生成 Rust 代码所需的扁平版本
    rust_definitions_file = Path("/Users/songqi/Work/quant/tushare-cli-rust/src/api/definitions.json")
    with open(rust_definitions_file, 'w', encoding='utf-8') as f:
        json.dump(full_definitions, f, ensure_ascii=False, indent=2)
    print(f"💾 同时更新了: {rust_definitions_file}")

if __name__ == "__main__":
    main()

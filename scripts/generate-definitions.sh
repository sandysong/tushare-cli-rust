#!/bin/bash
# 从 TypeScript 导出 API 定义为 JSON

set -e

TS_PROJECT_PATH="${1:-../tushare-cli}"
OUTPUT_FILE="src/api/definitions.json"

echo "Generating API definitions from TypeScript project..."

# 检查 TypeScript 项目是否存在
if [ ! -d "$TS_PROJECT_PATH" ]; then
    echo "Error: TypeScript project not found at $TS_PROJECT_PATH"
    exit 1
fi

# 使用 Bun 导出 API 定义
bun run -e "
import { API_REGISTRY } from '${TS_PROJECT_PATH}/src/api/definitions-generated.ts';
import { writeFileSync } from 'fs';

// 将 API 注册表转换为扁平结构
const flatApis = {};
for (const [category, apis] of Object.entries(API_REGISTRY)) {
  for (const api of apis) {
    flatApis[api.name] = api;
  }
}

writeFileSync('${OUTPUT_FILE}', JSON.stringify(flatApis, null, 2));
console.log(\`Exported \${Object.keys(flatApis).length} API definitions to ${OUTPUT_FILE}\`);
"

echo "API definitions exported successfully!"

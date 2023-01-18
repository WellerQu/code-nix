# code-nix
The code generator by templates from swagger yaml

模版文件与模版配置文件

模版文件是用于生成代码的格式化模版，通过模版引擎将元数据填入模版中，生成文件内容。模版语法与 [handlebars](https://handlebarsjs.com/) 相同。

模版片段示例

```text
interface {{model_name}} {
  {{#each fields}}
    {{this.field_name}} {{this.field_type}}
  {{/each}}
}
```

模版配置片段示例

```text
[basic]
file_name = path/to/model/event.ts
desc = "auto generate by code-nix"
```

元数据片段示例

```json
{
  "model": "Event",
  "fields": [
    {
      "field_name": "created_at",
      "field_type": "number"
    },
    {
      "field_name": "updated_at",
      "field_type": "number"
    }
  ]
}
```

输出文件示例

```typescript
// path/to/model/event.ts 模版配置中决定文件位置
interface Event {
  created_at: number
  updated_at: number
}
```
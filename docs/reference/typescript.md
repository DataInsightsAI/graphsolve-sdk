# TypeScript client

Install with `npm install @graphsolve/sdk`. The package ships its type
declarations, so your editor shows every tool's arguments and the allowed values
of each fixed vocabulary as you type.

- Package: [`@graphsolve/sdk` on npm](https://www.npmjs.com/package/@graphsolve/sdk)
- Source: [`typescript/`](https://github.com/DataInsightsAI/graphsolve-sdk/tree/main/typescript)

Tool and argument names are snake_case, exactly as in the API, so the
[tool reference](tools/index.md) applies unchanged. Options the client defines
itself, such as `baseUrl` and `maxRetries`, are camelCase.

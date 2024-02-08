### Endpoints
#### Transações
**Requisição**
*POST /clientes/[id]/transacoes*
```json
{
    "valor": 1000,
    "tipo" : "c",
    "descricao" : "descricao"
}
```

- **[id]** (na URL) deve ser um número inteiro representando a identificação do cliente.
- **valor** deve um número inteiro positivo que representa centavos (não vamos trabalhar com frações de centavos). Por exemplo, R$ 10 são 1000 centavos.
- **tipo** deve ser apenas c para crédito ou d para débito.
- **descricao** deve ser uma string de 1 a 10 caractéres.

**Resposta**
```json
{
    "limite" : 100000,
    "saldo" : -9098
}
```

#### Extrato
**Requisição**
*GET /clientes/[id]/extrato*
- [id] (na URL) deve ser um número inteiro representando a identificação do cliente.

**Resposta**
```json
{
  "saldo": {
    "total": -9098,
    "data_extrato": "2024-01-17T02:34:41.217753Z",
    "limite": 100000
  },
  "ultimas_transacoes": [
    {
      "valor": 10,
      "tipo": "c",
      "descricao": "descricao",
      "realizada_em": "2024-01-17T02:34:38.543030Z"
    },
    {
      "valor": 90000,
      "tipo": "d",
      "descricao": "descricao",
      "realizada_em": "2024-01-17T02:34:38.543030Z"
    }
  ]
}
```
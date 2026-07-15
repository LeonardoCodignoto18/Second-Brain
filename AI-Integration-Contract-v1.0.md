# AI Integration Contract v1.0

**Produto:** Second Brain OS  
**Status:** Congelado — contrato normativo da integração com IA  
**Versão:** 1.0  
**Data:** 15/07/2026  
**Origem:** AVR-01 — Architecture Validation Review — AI Integration Layer  
**Natureza:** contrato lógico-técnico complementar, equivalente a um ADR expandido

## 1. Finalidade, alcance e precedência

Este documento explicita como mecanismos de IA podem conversar com o Second Brain OS sem obter autoridade sobre dados, regras, consentimento ou execução. Ele não cria arquitetura, domínio, funcionalidade ou requisito de produto.

Permanecem normativos e inalterados:

1. PRD v1.2;
2. Arquitetura Conceitual v1.0;
3. Arquitetura Lógica v1.0;
4. Arquitetura Técnica v1.1;
5. ADR-007 — IA por propostas sem mutação;
6. ADR-008 — fronteira única de consentimento e egress.

Em caso de conflito, os documentos acima prevalecem. Este contrato restringe implementações possíveis; nunca amplia a autoridade da IA.

### 1.1 Escopo

O contrato abrange providers remotos, providers locais, modelos locais, adapters, transporte, consentimento, contexto, propostas, capacidades, erros, streaming, function calling, tool calling e a futura fronteira MCP.

### 1.2 Fora de escopo

- escolha de provider ou modelo;
- qualidade, custo, benchmark e ranking;
- implementação de adapters;
- implementação de MCP ou agentes;
- protocolos físicos, SDKs e frameworks;
- alteração dos nove tipos de proposta;
- alteração dos 17 domínios.

## 2. Invariantes permanentes

**AIC-INV-001.** Nenhum domínio conhece provider, modelo, SDK, prompt, token, endpoint, streaming, tool call ou formato de resposta externo.

**AIC-INV-002.** Todo resultado assistido que alcance o produto é uma Proposta Canônica validada; resposta de provider nunca é comando.

**AIC-INV-003.** Provider, adapter, transporte, MCP e ferramentas não possuem acesso direto ao dispatcher, banco de dados, repositórios, domínios ou memória.

**AIC-INV-004.** A IA não altera estado. Somente handlers de aplicação existentes podem solicitar mutações aos domínios, após decisão humana ou política determinística já autorizada pelo produto.

**AIC-INV-005.** Todo destino externo depende de autorização válida emitida pela Fronteira de Consentimento e verificada pelo único proprietário do transporte.

**AIC-INV-006.** Provider Adapter não possui rede, credenciais, persistência operacional nem capacidade de execução.

**AIC-INV-007.** Context Engine não conhece provider, prompt, token, política de egress, permissão ou destino.

**AIC-INV-008.** Decision Engine permanece local, determinístico e superior a qualquer sugestão do modelo nas restrições inegociáveis.

**AIC-INV-009.** Saída parcial, texto livre, tool call, resposta tardia ou resposta inválida não pode se tornar proposta aplicável.

**AIC-INV-010.** Trocar provider não migra nem reinterpreta a vida do usuário.

## 3. Vocabulário canônico

| Termo | Definição normativa |
|---|---|
| Provider | Implementação capaz de realizar uma operação de inferência segundo este contrato |
| Modelo | Recurso de inferência selecionado dentro de um provider; não é entidade de domínio |
| Provider Adapter | Tradutor puro entre contratos canônicos e representação específica do provider |
| Provider local | Provider executado dentro da fronteira local confiável definida para o aplicativo |
| Provider remoto | Provider cuja inferência exige transmissão para destino externo |
| Destino externo | Serviço, processo ou endpoint fora da fronteira local confiável |
| AI Orchestrator | Coordenador técnico existente que escolhe operação, capacidade, adapter e política operacional |
| External Transport Gateway | Único componente de infraestrutura autorizado a abrir conexões externas da camada de IA |
| Authorization Grant | capacidade local, opaca e temporária que autoriza uma transmissão específica |
| Context Package | seleção local transitória produzida sem aplicar política de egress |
| Authorized Context View | visão minimizada e autorizada para uma tentativa remota específica |
| Provider Request | requisição canônica entregue ao adapter para tradução |
| Provider Result | resultado canônico bruto normalizado pelo adapter, ainda não aplicável |
| Canonical Proposal | uma das nove propostas existentes, validada e independente de provider |

## 4. Provider Contract

### 4.1 Porta canônica

Todo provider é acessado por uma porta interna neutra com quatro responsabilidades conceituais:

1. declarar capacidades;
2. preparar representação específica a partir de uma Provider Request;
3. interpretar resposta específica como Provider Result;
4. cooperar com cancelamento e lifecycle segundo suas capacidades declaradas.

O contrato não pressupõe REST, HTTP, SDK, processo local, socket, JSON físico ou qualquer protocolo específico.

### 4.2 Provider Request

Uma requisição canônica contém, no mínimo:

| Campo | Regra |
|---|---|
| contract_version | versão deste contrato |
| request_id | identidade única da solicitação lógica |
| attempt_id | identidade única da tentativa |
| correlation_id | correlação ponta a ponta |
| operation | uma operação assistida reconhecida |
| proposal_schema | tipo e versão da Proposta Canônica esperada |
| context_view | contexto local ou visão autorizada, com proveniência e ausências |
| deterministic_constraints | restrições locais que nunca podem ser relaxadas |
| output_requirements | forma canônica esperada e limites aplicáveis |
| capability_requirements | capacidades mínimas exigidas |
| deadline | limite lógico absoluto da tentativa |

Provider Request não contém credencial, Authorization Grant, objeto de domínio mutável, handler, conexão, callback de mutação ou referência ao banco.

### 4.3 Provider Result

O adapter normaliza a saída para exatamente um destes resultados:

- `completed`: candidato completo a validação;
- `refused`: provider recusou a operação;
- `failed`: erro canônico;
- `cancelled`: cancelamento observado;
- `incomplete`: término sem resultado completo e validável.

Um `completed` ainda não é proposta. Ele deve passar pelo Proposal Validator e pelo Decision Engine antes de originar uma Canonical Proposal apresentável.

### 4.4 Responsabilidades do Provider Adapter

O adapter deve:

- traduzir requisição e resposta;
- declarar capacidades honestamente;
- mapear erros para a taxonomia canônica;
- rejeitar comportamento incompatível;
- preservar request, attempt e correlation IDs;
- impedir que tipos específicos atravessem sua fronteira;
- tratar todo conteúdo devolvido como não confiável.

O adapter não pode:

- abrir rede;
- acessar credenciais;
- consultar domínios;
- persistir conversas ou respostas;
- alterar contexto autorizado;
- escolher consentimento;
- executar tools;
- emitir comandos;
- aplicar retry por conta própria;
- degradar silenciosamente uma capacidade obrigatória.

### 4.5 Lifecycle

Estados conceituais de uma tentativa:

`prepared → authorized quando remoto → dispatched → receiving opcional → completed | failed | cancelled | expired`

Somente o AI Orchestrator inicia, cancela ou encerra logicamente uma tentativa. Adapter e transporte reportam fatos; não decidem fallback, retry ou aplicação.

### 4.6 Versionamento e compatibilidade

- O contrato usa versão explícita.
- Adapters declaram versões mínima e máxima suportadas.
- Campo desconhecido nunca altera semântica silenciosamente.
- Remoção, renomeação ou mudança semântica exige nova versão incompatível.
- Capacidades opcionais usam negociação explícita, não detecção por falha.
- Dados pessoais não persistem versão, modelo ou objetos do provider como parte de sua identidade.

## 5. Transport Contract

### 5.1 Proprietário único

O **External Transport Gateway**, responsabilidade interna da infraestrutura já prevista para AI Gateway e Fronteira de Consentimento, é o único proprietário da capacidade física de abrir conexões externas de IA.

**AIC-TRN-001.** Provider Adapters não recebem cliente de rede, socket, credencial, segredo, proxy irrestrito ou função genérica de envio.

**AIC-TRN-002.** SDK que abre conexões internamente não pode ser executado dentro de Provider Adapter. Se usado, deve ficar encapsulado pelo Transport Gateway e sujeito às mesmas verificações; sua existência não pode conceder bypass.

**AIC-TRN-003.** O Transport Gateway aceita somente destino registrado, requisição preparada, pacote autorizado e Authorization Grant válido.

### 5.2 Fluxo remoto obrigatório

1. AI Orchestrator solicita contexto para uma finalidade.
2. Context Engine produz Context Package local.
3. Fronteira de Dados classifica e aplica política de transmissão.
4. Consentimento produz Authorized Context View ou bloqueia a operação.
5. Consentimento emite Authorization Grant.
6. Provider Adapter prepara representação específica sem enviá-la.
7. Transport Gateway verifica grant, destino, fingerprint, validade e tentativa.
8. Transport Gateway injeta credencial e executa a transmissão.
9. Resposta bruta retorna ao adapter.
10. Adapter normaliza para Provider Result.
11. Validadores locais produzem ou rejeitam Canonical Proposal.

Nenhuma etapa pode ser omitida por retry, fallback, streaming ou troca de provider.

### 5.3 Credenciais e destinos

- Credenciais permanecem no núcleo protegido e são acessíveis somente pelo transporte/credential broker autorizado.
- Adapter pode referenciar uma identidade lógica de destino; nunca recebe segredo.
- Redirecionamento para destino não autorizado falha fechado.
- Provider, host, região ou classe de retenção materialmente diferente exige autorização compatível.
- Erros externos são saneados antes de diagnóstico ou UI.

### 5.4 Retry e idempotência de transporte

Retry é decisão do AI Orchestrator. Um Authorization Grant define número máximo de tentativas e só permite replay do mesmo fingerprint. Mudança de payload, provider, destino, finalidade, operação ou categorias requer novo grant.

Resposta de tentativa anterior não pode vencer uma tentativa posterior. O Orchestrator aceita no máximo um resultado terminal para o request lógico e rejeita respostas tardias pelo `attempt_id`.

## 6. Consent Contract

### 6.1 Authorization Grant

Authorization Grant é uma capacidade opaca, não falsificável pela camada solicitante, emitida localmente pela Fronteira de Consentimento e consumida somente pelo Transport Gateway. Não é enviado ao provider nem registrado integralmente em logs.

O grant é vinculado a:

| Vinculação | Conteúdo mínimo |
|---|---|
| provider | identidade lógica e versão de política material conhecida |
| destination | destino/serviço e classe remota |
| purpose | finalidade visível ao usuário |
| operation | operação assistida específica |
| categories | conjunto exato de categorias autorizadas |
| package_fingerprint | fingerprint canônico do conteúdo autorizado |
| validity | emissão, expiração e condição de revogação |
| retention_policy | política externa apresentada ou identificador de sua versão |
| request scope | request_id, correlação e limite de tentativas |

### 6.2 Regras de validade

O grant falha fechado quando:

- expirado ou revogado;
- já consumido além do limite;
- provider ou destino diverge;
- finalidade ou operação diverge;
- qualquer categoria adicional aparece;
- fingerprint diverge;
- política material de retenção mudou;
- request/attempt não pertence ao escopo;
- relógio ou estado de consentimento não pode ser validado com segurança.

### 6.3 Troca de provider

Autorização nunca é transferida implicitamente. Trocar provider ou destino invalida o grant anterior. Permissão persistente por categoria pode reduzir fricção somente quando o PRD permitir, mas a transmissão concreta exige novo grant e transparência do serviço/finalidade aplicáveis.

### 6.4 Cancelamento e revogação

- Revogação bloqueia novos grants e tentativas ainda não despachadas.
- Cancelamento após envio é melhor esforço e não promete apagar processamento externo.
- Resposta recebida após revogação/cancelamento não se torna proposta aplicável.
- Auditoria registra metadados mínimos: serviço, finalidade, categorias, referências, horário, resultado e erro canônico; nunca duplica o payload por padrão.

## 7. Context Contract

### 7.1 Separação obrigatória de responsabilidades

| Etapa | Proprietário | Conhece egress? | Saída |
|---|---|---:|---|
| Seleção | Context Engine | Não | Context Package específico por finalidade |
| Classificação | Fronteira de Dados | Sim, sem decidir consentimento | categorias e sensibilidade |
| Política de transmissão | Consentimento/Fronteira | Sim | permitido, bloqueado ou consentimento pendente |
| Minimização remota | Preparador de Pacote Autorizado | Sim | Authorized Context View |
| Suficiência | Regra pura da operação, invocável antes/depois | Não conhece o motivo da ausência | suficiente, insuficiente ou degradado |

### 7.2 Context Engine

O Context Engine pode:

- receber finalidade e requisitos de informação;
- consultar proprietários por interfaces aprovadas;
- separar fatos, preferências e inferências confirmadas;
- marcar origem, atualidade, confiança e ausências;
- limitar contexto à finalidade;
- calcular suficiência por requisitos semânticos.

O Context Engine não pode conhecer:

- provider ou modelo;
- endpoint ou destino;
- consentimento;
- regra “nunca enviar”;
- categoria autorizada para transmissão;
- prompt, tokens ou context window;
- retenção do provider;
- formato físico de requisição.

### 7.3 Suficiência após minimização

A Fronteira devolve uma visão com categorias presentes e ausentes, sem revelar ao Context Engine a política que causou a ausência. A mesma regra pura de suficiência é reaplicada. Se insuficiente, a operação usa fallback local ou pede decisão permitida; nunca completa lacunas com invenção.

### 7.4 Context window e tamanho

Limite do provider não altera seleção de domínio. O Orchestrator compara o tamanho lógico com a capacidade declarada e solicita uma visão reduzida por regras canônicas de prioridade. Truncamento arbitrário é proibido. Restrições determinísticas, categorias ausentes, proveniência e instruções de segurança nunca são removidas silenciosamente.

## 8. Proposal Contract

### 8.1 Forma única

Os nove tipos congelados permanecem os únicos resultados assistidos reconhecidos:

1. InterpretaçãoDeCaptura;
2. Prioridades;
3. Plano;
4. Agora;
5. Replanejamento;
6. Inferência;
7. PerguntaMínima;
8. ResumoEncerramento;
9. Explicação.

Provider não cria décimo tipo, comando, evento ou entidade.

### 8.2 Envelope canônico

Toda Canonical Proposal contém, no mínimo:

| Campo | Regra |
|---|---|
| proposal_id | identidade local gerada pelo sistema |
| proposal_type | um dos nove tipos |
| schema_version | versão do schema canônico |
| origin | assistida por modelo, sem objeto específico do provider |
| request_id | solicitação que a originou |
| source_fingerprint | versão/fingerprint das fontes locais |
| considered_categories | categorias efetivamente presentes |
| missing_categories | categorias ausentes relevantes |
| confidence | indicação calibrada, nunca autoridade |
| rationale | fatores relevantes e limitações, sem raciocínio interno integral |
| validity | expiração temporal e condições de invalidação |
| references | somente referências locais existentes e validadas |
| payload | conteúdo específico do schema canônico |
| forbidden_effects | efeitos que a proposta não autoriza |

IDs de response, conversation, thread, tool ou modelo do provider podem existir em diagnóstico transitório redigido, mas não integram a proposta ou os dados pessoais.

### 8.3 Pipeline obrigatório

`Provider Result → parse local → schema canônico → referências → restrições determinísticas → validade/fingerprint → Canonical Proposal | rejeição`

Validação declarada pelo provider não substitui nenhuma validação local. Reparo automático que altere significado exige nova tentativa autorizada; coerção silenciosa é proibida.

### 8.4 Aplicação

Canonical Proposal continua sendo rascunho. Somente o fluxo existente de decisão humana e comandos do domínio proprietário pode aplicá-la. Provider, adapter, validador e Orchestrator não invocam comandos mutantes.

## 9. Provider Capability Contract

### 9.1 Manifesto neutro

Cada adapter fornece um manifesto de capacidades versionado e verificável localmente:

| Capacidade | Semântica canônica |
|---|---|
| structured_output | consegue produzir candidato estruturado |
| schema_constraint | nível: nativo, induzido ou ausente |
| streaming | nenhum, progresso, conteúdo parcial ou eventos estruturados |
| cancellation | não suportado, solicitado ou confirmado |
| timeout_control | suportado pelo transporte/provider ou apenas local |
| retry_semantics | suporte a idempotency key e condições declaradas |
| function_serialization | pode usar função apenas como codec de proposta |
| tool_emission | pode emitir tool call; no MVP deve ser rejeitado |
| context_capacity | limite declarado com unidade canônica e grau de confiança |
| input_size_limit | limite efetivo conhecido |
| output_size_limit | limite efetivo conhecido |
| metadata | uso, finish reason e identificadores operacionais disponíveis |
| local_execution | execução classificada como local ou remota |

### 9.2 Negociação

- A operação declara requisitos mínimos.
- Orchestrator seleciona somente adapter compatível.
- Capacidade ausente produz `provider_incompatible`; não existe degradação silenciosa.
- Structured output induzido continua aceitável somente se o validador local aprovar.
- Tool emission não concede Tool Boundary.
- Limites podem ser calibrados, mas suas unidades e significado devem permanecer canônicos.

## 10. Error Taxonomy

Todo erro externo, local, de adapter ou transporte é convertido para uma categoria canônica antes de alcançar Orchestrator, fallback, auditoria ou UI.

| Código canônico | Significado | Retry padrão |
|---|---|---|
| context_insufficient | contexto permitido não sustenta a operação | não; fallback/pergunta |
| authorization_required | consentimento ainda não existe | não automático |
| authorization_denied | usuário/política bloqueou | não |
| authorization_expired | grant não é mais válido | somente novo fluxo autorizado |
| destination_not_allowed | destino não está registrado/autorizado | não |
| provider_incompatible | capacidades não atendem à operação | não naquele adapter |
| provider_unavailable | serviço/processo indisponível | no máximo política central |
| rate_limited | limite externo atingido | somente com espera permitida e deadline válido |
| timeout | deadline terminou | no máximo política central |
| cancelled | cancelamento solicitado/observado | não |
| transport_failure | conexão falhou antes de resposta utilizável | retry central limitado |
| authentication_failure | credencial externa recusada | não automático |
| response_partial | fluxo terminou incompleto | não aplicar; fallback |
| response_late | tentativa já encerrada/substituída | descartar |
| response_invalid | resposta não interpretável | não com mesmo input |
| schema_invalid | candidato viola schema canônico | não com mesmo input |
| constraint_violation | viola regra determinística/referência | não; fallback |
| safety_refusal | provider recusou por política | não automático |
| context_overflow | contexto excede capacidade declarada | nova preparação, nunca truncamento arbitrário |
| protocol_failure | adapter não conseguiu traduzir protocolo | não até correção/compatibilidade |
| internal_failure | falha local inesperada e saneada | fallback seguro |

### 10.1 Regras

- Mensagem bruta de provider não é exibida nem persistida sem saneamento.
- Retry é determinado por código, deadline, attempts e ausência de efeito terminal.
- No máximo um retry transitório antes de qualquer resposta, conforme política técnica congelada.
- Erro de validação não aciona retry idêntico.
- Falha sempre preserva o ciclo determinístico/manual.

## 11. Streaming Contract

### 11.1 Tipos de streaming

- **Sem streaming:** resposta completa única.
- **Progresso:** estados neutros sem conteúdo semântico, como conectando/recebendo/validando.
- **Conteúdo parcial:** fragmentos não confiáveis mantidos no adapter/buffer.
- **Eventos estruturados:** eventos específicos normalizados internamente, ainda não propostas.

### 11.2 Pipeline

`stream externo → buffer transitório → detecção de término → montagem completa → validação local → Canonical Proposal`

**AIC-STR-001.** Fragmento nunca é proposta, fato, memória, comando ou atualização de plano.

**AIC-STR-002.** Domínios não recebem tokens nem eventos do provider.

**AIC-STR-003.** UI pode receber apenas progresso neutro por padrão. Streaming textual futuro deve ser explicitamente provisório e não autoritativo.

**AIC-STR-004.** Buffer é limitado, transitório, redigido de logs e descartado após conclusão, falha ou cancelamento.

**AIC-STR-005.** Fim de stream sem marcador terminal válido produz `response_partial`.

### 11.3 Cancelamento e resposta tardia

Cancelar fecha a tentativa logicamente, instrui transporte/provider conforme capacidade e descarta buffer. Mesmo que o provider não confirme cancelamento, qualquer resultado posterior recebe `response_late` e não atravessa o validador como tentativa ativa.

## 12. Local Provider Contract

### 12.1 Classificação

“Provider local”, “modelo local” e “destino externo” são conceitos distintos:

- Provider local pode hospedar um ou mais modelos locais.
- Provider remoto sempre implica destino externo.
- Um processo na mesma máquina não é automaticamente confiável; deve estar dentro da fronteira local explicitamente aprovada.
- Endpoint localhost de software de terceiros é classificado por política de destino, não apenas pelo endereço.

### 12.2 Fluxo local

Provider genuinamente local usa Provider Contract, Capability Contract, Proposal Contract, Error Taxonomy e validação idênticos. Não recebe Authorization Grant de transmissão externa e não gera registro de egress.

Ainda são obrigatórios:

- finalidade específica;
- seleção mínima de contexto;
- proveniência e suficiência;
- isolamento de credenciais quando existirem;
- proposta sem mutação;
- descarte de temporários;
- proteção contra prompt/tool injection.

### 12.3 Reclassificação

Se provider local passar a encaminhar, sincronizar, registrar remotamente ou depender de serviço externo, ele é tratado como remoto. Mudança de classe invalida autorização/configuração anterior e exige transparência.

## 13. MCP Boundary

MCP permanece fora do MVP e não será implementado no SPK-05.

**AIC-MCP-001.** MCP não é Provider Adapter.

**AIC-MCP-002.** MCP terá fronteira e adapters próprios, sem acesso direto a domínios, banco, repositórios, credenciais ou dispatcher.

**AIC-MCP-003.** Recursos MCP consumidos serão fontes externas não confiáveis, sujeitos a consentimento, proveniência, classificação, minimização e sanitização.

**AIC-MCP-004.** Tools MCP nunca serão executadas diretamente a partir de saída de modelo.

**AIC-MCP-005.** Expor MCP futuramente significa adaptar consultas e comandos de aplicação já existentes, preservando validação, autenticação, autorização, confirmação, idempotência e auditoria. Não significa expor domínios.

**AIC-MCP-006.** Prompts fornecidos por MCP são conteúdo externo não confiável e nunca substituem instruções ou políticas locais.

### 13.1 Evolução futura

Consumo de recursos, consumo de tools e exposição de MCP são três capacidades separadas. Cada uma exigirá decisão técnica e validação próprias. Nenhuma altera os 17 domínios se continuar subordinada às portas de aplicação existentes.

## 14. Tool Boundary

### 14.1 Function calling

Function calling pode ser usado dentro do adapter exclusivamente como codec para construir candidato a uma das nove propostas. O nome da “função” não referencia handler real; seus argumentos permanecem saída não confiável e passam por validação integral.

### 14.2 Tool calling

No MVP, qualquer tool call emitida pelo provider resulta em `provider_incompatible` ou `response_invalid`, conforme a capacidade negociada. Ela não pode alcançar:

- dispatcher;
- command handler;
- banco;
- filesystem;
- rede adicional;
- domínio;
- memória;
- notificações;
- integração Windows.

### 14.3 Futuro

Uma Tool Boundary futura deverá exigir catálogo local allowlisted, schemas canônicos, classificação de leitura/mutação, consentimento, confirmação, idempotência, timeout, resultado não confiável e auditoria. Até decisão formal, não existe executor de tools.

## 15. Segurança e comportamento adversarial

| Risco | Regra contratual |
|---|---|
| Prompt injection | contexto e fontes externas são dados; nunca instruções de autoridade |
| Jailbreak | pode afetar candidato, não controles locais ou execução |
| Prompt leakage | prompts, contexto integral e mensagens internas não entram em logs/auditoria por padrão |
| Data leakage | somente Transport Gateway com grant válido transmite |
| Schema poisoning | schema é local, versionado e nunca aceito do provider/MCP |
| Tool injection | tools rejeitadas no MVP |
| Hallucination | referências, fatos e restrições são validados localmente |
| Replay | request/attempt/fingerprint e consumo do grant impedem repetição indevida |
| Timeout/cancelamento | tentativa encerrada rejeita resposta tardia |
| Context overflow | redução semântica explícita ou falha; truncamento arbitrário proibido |
| Cross-provider behavior | normalização por capacidades, resultados e erros canônicos |

Conteúdo de provider, MCP, documento ou usuário jamais pode modificar este contrato em runtime.

## 16. Provider Independence

| Provider/classe | O que muda | O que não muda |
|---|---|---|
| OpenAI | adapter, manifesto, destino e configuração | contexto, consentimento, propostas, validação, domínios |
| Anthropic | adapter, manifesto, destino e configuração | idem |
| Google Gemini | adapter, manifesto, destino e configuração | idem |
| OpenRouter | adapter e identidade/política do intermediário; consentimento identifica o serviço | idem |
| Ollama local | adapter e classe local validada | propostas, validação, domínios; sem egress se genuinamente local |
| LM Studio local | adapter e classe local validada | idem |
| Provider futuro | adapter + capacidades + destino quando remoto | todos os contratos canônicos |

Adicionar provider compatível não altera:

- PRD;
- Arquitetura Conceitual;
- Arquitetura Lógica;
- Arquitetura Técnica;
- 17 domínios;
- Decision Engine;
- catálogo de propostas;
- regras de consentimento;
- persistência pessoal.

Se um provider exigir mutação direta, schema próprio nos domínios, tool executor, memória externa autoritativa, consentimento implícito ou transporte fora do gateway, ele é incompatível e não deve ser integrado.

## 17. Conformidade e testes contratuais

Todo adapter deve passar pela mesma suíte, independentemente do provider:

1. nenhum tipo específico atravessa a porta;
2. adapter funciona sem rede e sem credencial próprias;
3. transporte recusa ausência, expiração e divergência do grant;
4. troca de provider/destino invalida grant;
5. fingerprint ou categoria divergente falha fechado;
6. contexto insuficiente não é preenchido por invenção;
7. nove schemas válidos e inválidos;
8. referências inexistentes e restrições determinísticas;
9. resposta parcial, tardia, cancelada e duplicada;
10. taxonomia de timeout, rate limit, indisponibilidade e incompatibilidade;
11. streaming nunca gera proposta parcial;
12. tool call é rejeitada;
13. prompt/tool/schema injection não alcança execução;
14. provider local não produz egress;
15. endpoint local reclassificado como remoto exige consentimento;
16. falha sempre aciona fallback determinístico/manual.

SPK-05 valida estes contratos; não decide sua forma.

## 18. Decisões congeladas e pontos calibráveis

### 18.1 Congelado

- portas e responsabilidades;
- transporte externo único;
- Authorization Grant vinculado;
- separação Context Engine/egress;
- propostas canônicas;
- capacidades e erros neutros;
- semântica de streaming;
- distinção local/remoto;
- MCP separado;
- tools rejeitadas no MVP;
- provider sem mutação.

### 18.2 Calibrável sem decisão arquitetural

- provider/modelo inicial;
- limites numéricos de entrada e saída;
- deadlines dentro da política já aprovada;
- tamanhos de buffer;
- formato físico dos envelopes;
- método de fingerprint e representação opaca do grant;
- política operacional de retry dentro do máximo congelado;
- mensagens de UX;
- seleção entre schema nativo e induzido;
- detalhes de diagnóstico redigido.

Calibração não pode enfraquecer invariantes.

## 19. Condições de revisão

Este contrato só deve ser revisto se:

- o PRD autorizar execução autônoma;
- tools ou MCP entrarem no escopo;
- colaboração/sync alterar a fronteira local;
- um provider indispensável não puder produzir Proposta Canônica;
- múltiplos processos exigirem nova autoridade de transporte;
- regulamentação exigir consentimento/retenção diferente;
- agentes deixarem de ser apenas geradores de propostas.

Mudança de provider, modelo, endpoint, SDK, protocolo ou suporte a streaming, isoladamente, não justifica alterar domínios ou este contrato.

## 20. Decisão final

Com este contrato, qualquer provider compatível pode ser integrado por adapter e infraestrutura já prevista, preservando Local First, Consentimento, Context Engine independente, Decision Engine determinístico e IA limitada a propostas.

Nenhuma decisão estrutural da camada de integração permanece aberta antes do SPK-05. Permanecem somente escolhas operacionais e evidências técnicas a serem medidas pelo spike.

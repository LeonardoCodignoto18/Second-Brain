# PRD 1.2 — Second Brain OS

**Status:** Aprovado e congelado para orientar arquitetura  
**Versão:** 1.2  
**Usuário do piloto:** Fundador do produto  
**Plataforma do MVP:** Windows  
**Estratégia de dados:** Local First  
**Escopo:** contrato de produto; não define arquitetura, tecnologias ou implementação

## 1. Finalidade deste documento

Este PRD consolida a visão oficial do Second Brain OS e a transforma em requisitos operacionais, verificáveis e priorizados.

As declarações estão classificadas como:

- **Princípio permanente:** regra que orienta todas as versões.
- **MVP obrigatório:** necessário para validar a hipótese principal.
- **MVP condicional:** incluído apenas quando um teste demonstrar que é necessário ao ciclo central.
- **Pós-MVP:** importante, mas não necessário para validar a hipótese.
- **Experimento futuro:** possibilidade que exige validação antes de virar requisito.

Em caso de conflito, prevalecem, nesta ordem:

1. segurança, privacidade e autonomia do usuário;
2. redução de carga mental;
3. funcionamento confiável do ciclo diário;
4. simplicidade da experiência;
5. qualidade acima da quantidade;
6. possibilidades futuras.

## 2. Resumo executivo

O Second Brain OS é uma plataforma pessoal de orientação e execução. Sua função é reduzir a carga mental e a quantidade de decisões conscientes necessárias para organizar e conduzir o dia.

O produto transforma tarefas, compromissos, disponibilidade, energia e objetivo semanal em até três prioridades e uma orientação atual chamada **Agora**. Ele ajuda o usuário a capturar o que não pode ser esquecido, planejar de forma realista, iniciar a próxima ação, adaptar o dia e refletir sem culpa.

A Inteligência Artificial conecta contexto e oferece recomendações explicadas, mas não controla o usuário. O sistema mantém utilidade essencial sem qualquer modelo de IA disponível.

A experiência principal do MVP utiliza IA externa mediante consentimento explícito. A lógica determinística existe como modo de continuidade e segurança, não como substituta da experiência principal.

O MVP será pessoal, exclusivo para Windows e Local First. Seu objetivo não é cobrir todas as áreas da vida, mas validar com excelência um ciclo diário completo.

## 3. Missão

Reduzir as decisões operacionais e a carga mental do usuário para que sua energia seja direcionada a pensar, aprender, criar, descansar e executar.

O produto não organiza apenas tarefas. Ele ajuda uma pessoa a conduzir a própria vida com clareza, foco, consistência e tranquilidade.

## 4. Visão de longo prazo

O Second Brain OS deverá evoluir para um sistema operacional pessoal que:

- acompanha o usuário durante anos;
- lembra o que seria esquecido;
- conecta contexto disperso;
- reconhece padrões com transparência;
- questiona planos ruins sem impor decisões;
- reduz alternância entre ferramentas;
- adapta-se às diferentes fases da vida;
- transforma o computador em um parceiro inteligente.

Essa visão não autoriza antecipar funcionalidades futuras no MVP.

## 5. Problema

O usuário distribui informações importantes entre memória, arquivos, anotações, calendários e aplicativos. Como consequência, precisa decidir repetidamente:

- o que merece atenção;
- o que fazer agora;
- como encaixar atividades no tempo disponível;
- o que foi esquecido;
- como reorganizar um dia que mudou;
- se suas ações continuam alinhadas ao que considera importante.

O resultado é fadiga de decisão, procrastinação, ansiedade, culpa, perda de foco e tempo gasto administrando a própria organização.

## 6. Usuário e contexto inicial

### 6.1 Usuário do MVP

O MVP será desenvolvido e validado exclusivamente pelo fundador: estudante e profissional de tecnologia com múltiplos estudos, projetos, tarefas, compromissos e objetivos.

### 6.2 Públicos futuros

- estudantes;
- profissionais de tecnologia;
- desenvolvedores;
- criadores de conteúdo;
- profissionais do conhecimento;
- empreendedores e gestores.

### 6.3 Exclusões do público inicial

O MVP não atende colaboração, equipes, clientes, organizações ou compartilhamento de projetos.

## 7. Hipótese falsificável do MVP

> Se o sistema transformar objetivo semanal, compromissos, disponibilidade, energia e tarefas em até três prioridades e uma orientação **Agora** explicada, o usuário reduzirá o tempo até iniciar uma atividade relevante, tomará menos decisões conscientes de organização ao longo do dia e perceberá menor carga mental após quatro semanas de uso.

A hipótese será avaliada após uma semana de linha de base e quatro semanas de uso diário.

## 8. Promessa central

O usuário não deve precisar se perguntar: **“O que faço agora?”**

Quando houver contexto suficiente, o sistema deve oferecer uma orientação clara. Quando não houver, deve reconhecer a incerteza e fazer a menor pergunta necessária.

## 9. Princípios permanentes

**PP-01 — Orientação acima de armazenamento.** Toda entidade e funcionalidade deve contribuir para reduzir carga mental, preservar contexto ou facilitar ação.

**PP-02 — Simplicidade para o usuário.** Sofisticação técnica só é válida quando melhora a experiência, confiabilidade, segurança ou capacidade real de evolução.

**PP-03 — Qualidade acima de quantidade.** Uma experiência menor e excelente prevalece sobre uma coleção de recursos medianos.

**PP-04 — Contexto acima de comandos.** Recomendações consideram o contexto disponível, não apenas a última solicitação.

**PP-05 — Controle do usuário.** A IA pode sugerir, explicar e discordar; a decisão final pertence ao usuário.

**PP-06 — Transparência.** Fatos, inferências, recomendações, ações e transmissões externas devem ser distinguíveis.

**PP-07 — Sem culpa.** O sistema não julga, infantiliza, ameaça ou associa produtividade ao valor pessoal.

**PP-08 — Local First.** Dados pessoais permanecem no dispositivo por padrão.

**PP-09 — Memória corrigível.** O usuário pode revisar, corrigir, rejeitar, arquivar e excluir o que o sistema acredita saber.

**PP-10 — Incerteza explícita.** Inferências nunca são apresentadas como fatos.

**PP-11 — Proatividade proporcional.** O sistema intervém apenas quando houver valor provável e respeita preferências, horários silenciosos e rejeições.

**PP-12 — Descanso legítimo.** Saúde, manutenção e recuperação podem ser prioridades; nem todo dia deve maximizar produção.

**PP-13 — Evolução em camadas.** Funcionalidades futuras não devem contaminar desnecessariamente o MVP.

**PP-14 — Independência sustentável.** O produto não deve ficar irreversivelmente preso a um fornecedor de IA.

**PP-15 — Reversibilidade.** Ações que alterem o plano ou os dados devem ser reversíveis sempre que possível.

## 10. Glossário oficial

**Caixa de entrada:** local universal para itens capturados e ainda não organizados.

**Item capturado:** conteúdo bruto que pode se tornar tarefa, compromisso, ideia ou anotação.

**Tarefa:** ação executável com resultado observável.

**Compromisso:** evento que reserva um período específico e restringe a disponibilidade.

**Ideia:** possibilidade registrada sem obrigação de execução.

**Anotação:** informação preservada para consulta ou contextualização.

**Objetivo semanal:** principal resultado que o usuário deseja promover durante a semana atual.

**Prioridade do dia:** orientação aprovada como uma das atividades mais relevantes do dia. Existem no máximo três.

**Plano diário:** combinação entre compromissos fixos e sequência flexível de orientações no tempo restante.

**Agora:** orientação atual. Pode ser tarefa, compromisso, preparação, pausa, descanso ou indicação consciente de que nenhuma ação deve ser iniciada.

**Projeto:** resultado composto que exige múltiplas tarefas. No MVP, é um contexto simples, não um espaço completo de gestão.

**Rotina permanente:** estrutura recorrente que influencia disponibilidade. No MVP, será representada somente pelos compromissos recorrentes indispensáveis ao planejamento.

**Fato:** informação declarada ou confirmada pelo usuário.

**Preferência:** comportamento desejado explicitamente pelo usuário.

**Observação:** evento registrado pelo sistema, como aceitação ou duração real.

**Inferência:** hipótese produzida a partir de observações; não é considerada verdadeira sem confirmação.

**Recomendação:** proposta explicada que não altera dados importantes por si só.

**Rascunho de plano:** conjunto de alterações propostas ainda não aprovadas.

**Carga mental percebida:** avaliação subjetiva do esforço necessário para lembrar, organizar e decidir o que fazer.

**Disponibilidade:** janela configurável de cada dia, reduzida pelos compromissos cadastrados e por exceções específicas daquela data.

**Plano útil:** plano sem conflitos silenciosos que contém compromissos válidos e, quando existirem atividades disponíveis, pelo menos uma orientação executável.

**Atividade relevante:** prioridade aprovada, tarefa vinculada ao objetivo semanal ou atividade explicitamente marcada como importante pelo usuário.

## 11. Modelo operacional do MVP

### 11.1 Ciclo diário

1. O usuário captura informações a qualquer momento.
2. No início do dia, informa energia e revisa mudanças essenciais.
3. O sistema considera objetivo semanal, compromissos, tarefas, duração e disponibilidade.
4. O sistema propõe no máximo três prioridades e explica os fatores relevantes.
5. O usuário aprova o plano como um conjunto ou o ajusta.
6. O sistema apresenta o **Agora**.
7. O usuário inicia, substitui, pausa ou encerra a orientação atual.
8. Mudanças relevantes podem gerar um novo plano em rascunho.
9. No encerramento, o sistema registra o resultado e prepara o próximo dia.

### 11.2 Modelo híbrido de planejamento

- Compromissos possuem início e término fixos.
- Tarefas são organizadas como sequência flexível no tempo restante.
- Tarefas não recebem obrigatoriamente horário exato.
- O sistema não deve criar sobreposição entre compromissos.
- Intervalos indisponíveis não podem receber tarefas.
- Uma tarefa só entra no plano quando houver duração suficiente ou quando o sistema declarar a incerteza.
- Alterações no **Agora** não obrigam a reorganização de todo o dia.
- A disponibilidade padrão é configurada como uma janela para cada dia da semana.
- Exceções podem ampliar, reduzir ou remover a disponibilidade de uma data específica.
- O dia operacional usa o fuso local do Windows e possui horário de transição configurável, com padrão à meia-noite.
- Mudanças de fuso não devem alterar silenciosamente a intenção original de um compromisso.

### 11.3 Modelo de tarefa

Campos essenciais:

- título;
- estado;
- duração estimada, quando necessária ao planejamento;
- prazo, quando existir;
- data desejada, quando existir;
- vínculo opcional com objetivo semanal ou projeto;
- categoria opcional de saúde, manutenção ou descanso;
- duração real, quando executada;
- histórico mínimo de mudanças.

Prazo, data desejada e duração são conceitos diferentes.

### 11.4 Ciclo de vida da tarefa

Estados permitidos no MVP:

1. **Caixa de entrada:** capturada, ainda não organizada.
2. **Planejada:** pronta e considerada em um plano.
3. **Em andamento:** iniciada, inclusive quando temporariamente interrompida.
4. **Concluída:** resultado encerrado pelo usuário.
5. **Adiada:** retirada do plano atual com intenção de reconsideração.
6. **Cancelada:** não deve mais ser executada.

Transições devem preservar histórico. O estado “Aguardando” fica fora do MVP.

### 11.5 Modelo de compromisso

Campos essenciais:

- título;
- data;
- hora inicial;
- hora final;
- recorrência simples, quando necessária à rotina;
- observação opcional.

Compromissos serão cadastrados manualmente no MVP.

No MVP, recorrência significa somente repetição semanal em dias selecionados. Ao editar ou remover uma ocorrência recorrente, o usuário deve escolher entre alterar somente aquela ocorrência ou toda a série.

### 11.5.1 Modelo mínimo de projeto

Projeto no MVP contém somente:

- nome;
- descrição opcional;
- estado ativo ou arquivado;
- vínculo opcional com tarefas.

Projetos não possuem página avançada, Kanban, documentos, percentual, cronograma ou automações no MVP.

### 11.5.2 Destino de ideias e anotações

Após a captura, ideias e anotações podem somente:

- ser convertidas em tarefa;
- ser arquivadas;
- ser excluídas.

Não existe módulo próprio de ideias ou anotações no MVP.

### 11.6 Objetivo semanal

- Existe no máximo um objetivo semanal principal ativo.
- O usuário pode alterá-lo com confirmação explícita.
- Uma tarefa não precisa contribuir para esse objetivo.
- Saúde, manutenção, descanso e compromissos permanecem legítimos.
- O sistema deve revelar conflitos entre o plano e o objetivo sem impor mudanças.

### 11.7 Prioridades

- O sistema sugere zero, uma, duas ou três prioridades.
- Nunca preenche o limite artificialmente.
- Compromissos obrigatórios não ocupam automaticamente uma posição.
- O usuário aprova as prioridades em uma única decisão de plano.
- Saúde, manutenção e descanso podem ser prioridades.
- O usuário pode aceitar todo o conjunto, substituir uma prioridade, remover uma prioridade, aceitar menos de três ou decidir depois.
- Ajustar uma prioridade não deve reiniciar todo o planejamento.

### 11.8 Regra do Agora

O **Agora** considera, nesta ordem contextual, sem transformar a lista em fórmula rígida:

1. compromisso iminente e preparação necessária;
2. prazo e risco de consequência;
3. objetivo semanal;
4. compatibilidade com o tempo disponível;
5. energia informada;
6. continuidade de atividade iniciada;
7. saúde, manutenção e necessidade de descanso.

Regras adicionais:

- somente uma orientação fica destacada;
- a razão da orientação deve ser acessível;
- o usuário pode substituí-la sem reorganizar o restante;
- pouca confiança exige transparência e, quando necessário, uma pergunta curta;
- o sistema pode indicar pausa, preparação ou ausência consciente de ação.

### 11.9 Replanejamento

O sistema pode oferecer replanejamento quando:

- uma tarefa exceder a duração estimada e, como consequência, ameaçar compromisso ou parte aprovada do plano;
- um compromisso for criado, removido ou alterado;
- uma prioridade for adiada;
- a energia informada mudar;
- deixar de existir tempo suficiente para o restante;
- o usuário declarar que o plano mudou;
- o usuário disser: “Hoje não vou conseguir seguir o plano.”

O replanejamento deve:

1. preservar compromissos válidos;
2. produzir um rascunho;
3. explicar mudanças relevantes;
4. solicitar uma única aprovação geral;
5. permitir ajustes antes da confirmação;
6. manter o plano anterior recuperável até a aprovação.

Os gatilhos são classificados como:

- **Imediatos:** conflito com compromisso iminente ou declaração explícita de que o plano não poderá ser seguido.
- **Discretos:** mudança de energia, prioridade adiada ou alteração sem consequência imediata.
- **Postergáveis:** duração excedida sem conflito próximo.

O sistema não deve interromper uma sessão produtiva por gatilho discreto ou postergável.

### 11.10 Estados da experiência inicial

Antes da aprovação do plano, a experiência apresenta check-in, contexto essencial e proposta de prioridades.

Depois da aprovação, o **Agora** torna-se o elemento dominante. Objetivo, compromissos e restante do plano permanecem acessíveis, mas visualmente secundários.

## 12. Escopo obrigatório do MVP

### 12.1 Captura universal

**CAP-01.** O usuário deve conseguir capturar texto sem classificar o item imediatamente.

**CAP-02.** A captura deve aceitar tarefa, compromisso, ideia e anotação.

**CAP-03.** O item deve ser preservado na Caixa de entrada mesmo quando a interpretação falhar.

**CAP-04.** A classificação sugerida não pode alterar silenciosamente informações críticas.

**CAP-05.** O usuário deve conseguir revisar e organizar itens posteriormente.

**CAP-06.** Ideias e anotações devem permitir somente conversão em tarefa, arquivamento ou exclusão.

### 12.2 Onboarding progressivo

**ONB-01.** O onboarding deve coletar apenas o contexto mínimo necessário para o primeiro plano útil.

**ONB-02.** O contexto mínimo contém objetivo semanal, compromissos próximos, disponibilidade básica e tarefas relevantes.

**ONB-03.** Energia, preferências e rotina podem ser aprofundadas progressivamente.

**ONB-04.** O usuário deve poder interromper o onboarding sem perder dados.

**ONB-05.** O produto deve entregar uma primeira orientação sem exigir o cadastro completo da vida.

**ONB-06.** O primeiro plano útil deve obedecer à definição oficial deste PRD.

### 12.3 Início do dia

**DAY-01.** O sistema deve oferecer início configurável com o Windows.

**DAY-02.** O usuário deve poder iniciar em primeiro plano, discretamente ou desativar a inicialização automática.

**DAY-03.** O check-in deve permitir energia alta, normal ou baixa e também ser ignorado.

**DAY-04.** O início do dia deve apresentar objetivo semanal, compromissos relevantes, prioridades propostas e **Agora**.

**DAY-05.** A tela inicial não deve expor toda a lista de pendências por padrão.

**DAY-06.** Antes da aprovação, a proposta de plano deve dominar a experiência; depois dela, o **Agora** deve assumir o foco principal.

### 12.4 Planejamento e prioridades

**PLN-01.** O sistema deve respeitar o modelo híbrido definido neste PRD.

**PLN-02.** Deve sugerir no máximo três prioridades.

**PLN-03.** Deve explicar os principais fatores usados na proposta.

**PLN-04.** O usuário deve aprovar o plano em uma única ação ou editá-lo antes da aprovação.

**PLN-05.** Uma tarefa sem duração pode permanecer capturada, mas o sistema deve solicitá-la antes do planejamento quando necessária à confiabilidade.

**PLN-06.** O plano deve continuar editável manualmente sem IA.

**PLN-07.** Compromissos não podem ser sobrepostos silenciosamente.

**PLN-08.** O usuário deve poder ajustar parcialmente as prioridades sem reiniciar o fluxo.

**PLN-09.** A disponibilidade deve resultar da janela diária configurada menos compromissos e exceções da data.

### 12.5 Agora

**NOW-01.** O sistema deve destacar somente uma orientação atual.

**NOW-02.** A orientação deve poder representar tarefa, compromisso, preparação, pausa, descanso ou nenhuma ação.

**NOW-03.** O usuário deve acessar a justificativa da orientação.

**NOW-04.** O usuário deve conseguir substituir o **Agora** sem replanejar todo o dia.

**NOW-05.** O sistema deve comunicar quando não possuir contexto suficiente.

### 12.6 Execução

**EXE-01.** O usuário deve iniciar e encerrar uma tarefa planejada.

**EXE-02.** Um cronômetro deve ser opcional.

**EXE-03.** Interrupções devem preservar estado e tempo registrado.

**EXE-04.** O Modo Foco deve exibir apenas orientação atual, propósito, tempo e controles essenciais.

**EXE-05.** O usuário deve conseguir sair do Modo Foco sem perder progresso.

### 12.7 Replanejamento

**RPL-01.** Os gatilhos definidos na seção 11.9 devem poder iniciar uma proposta.

**RPL-02.** A proposta deve existir como rascunho até ser aprovada.

**RPL-03.** Mudanças devem ser explicadas de forma breve.

**RPL-04.** Uma única aprovação deve aplicar o conjunto.

**RPL-05.** O usuário deve poder rejeitar sem insistência ou julgamento.

**RPL-06.** Gatilhos discretos ou postergáveis não devem interromper uma sessão produtiva.

### 12.8 Encerramento do dia

**END-01.** O planejamento do dia seguinte deve ser recomendado, não obrigatório.

**END-02.** O horário deve ser configurável.

**END-03.** O fluxo deve revisar concluído, pendências, mudanças e objetivo semanal.

**END-04.** Pendências não devem ser movidas silenciosamente.

**END-05.** O usuário deve poder encerrar sem responder perguntas opcionais.

### 12.9 Retomada

**RET-01.** Após ausência, o sistema deve oferecer revisão de contexto desatualizado.

**RET-02.** A linguagem não deve representar a ausência como falha.

**RET-03.** Recomendações antigas não devem ser reaplicadas sem nova avaliação.

### 12.10 Notificações mínimas

**NOT-01.** Notificações devem ser configuráveis e desativáveis.

**NOT-02.** O MVP pode notificar compromisso próximo, orientação planejada e convite para encerramento.

**NOT-03.** Notificações repetidas após rejeição ou silêncio são proibidas.

**NOT-04.** O usuário deve configurar horários silenciosos.

**NOT-05.** O produto não deve usar urgência artificial ou linguagem culpabilizante.

**NOT-06.** Intervenções proativas do MVP ficam limitadas a check-in matinal, compromisso próximo, replanejamento com consequência real e convite configurável para encerramento.

**NOT-07.** Outras recomendações devem permanecer dentro do aplicativo.

## 13. Funcionamento com e sem IA

A experiência-alvo do MVP utiliza um serviço externo de IA autorizado pelo usuário. O modo sem modelo preserva continuidade, acesso e segurança, mas não representa a experiência principal usada para validar o diferencial do produto.

| Capacidade | Sem modelo | Modelo local futuro | IA externa permitida |
|---|---|---|---|
| Capturar, classificar manualmente e editar | Completo | Completo | Completo |
| Compromissos e disponibilidade | Completo | Completo | Completo |
| Objetivo semanal | Completo | Completo | Completo |
| Planejamento manual | Completo | Completo | Completo |
| Ordenação determinística básica | Disponível e identificada | Disponível | Disponível |
| Sugestão contextual de prioridades | Limitada por regras | Conforme capacidade | Completa dentro das permissões |
| Agora manual | Completo | Completo | Completo |
| Agora sugerido | Limitado por regras | Conforme capacidade | Completo dentro das permissões |
| Modo Foco e cronômetro | Completo | Completo | Completo |
| Replanejamento manual | Completo | Completo | Completo |
| Replanejamento explicado | Limitado | Conforme capacidade | Completo dentro das permissões |
| Registro de observações | Completo | Completo | Completo |
| Inferências comportamentais | Indisponível ou limitada | Conforme capacidade | Completa dentro das permissões |
| Encerramento do dia | Completo em formulário | Conversacional | Conversacional |

**AI-01.** O produto deve identificar quando uma sugestão é determinística ou assistida por modelo.

**AI-02.** Indisponibilidade de IA não pode impedir acesso ou edição dos dados.

**AI-03.** A lógica determinística pode considerar compromisso, prazo, prioridade declarada e duração disponível.

**AI-04.** Recomendações de IA devem apresentar fatores relevantes, não raciocínio interno integral.

**AI-05.** A IA deve reconhecer contexto insuficiente.

**AI-06.** Rejeições não devem gerar insistência automática.

**AI-07.** O usuário deve controlar frequência, proatividade, estilo e intensidade.

**AI-08.** O MVP não promete equivalência entre modelos locais e externos.

**AI-09.** O MVP pode implementar um único fornecedor externo; independência significa preservar dados e regras centrais de modo que uma substituição futura não exija migração da vida do usuário.

**AI-10.** Cada recomendação deve identificar quais categorias de contexto foram consideradas e comunicar limitações relevantes.

**AI-11.** Ausência de uma categoria crítica deve reduzir a confiança, acionar lógica local complementar ou impedir uma recomendação que pareça indevidamente completa.

**AI-12.** Inferências do MVP ficam limitadas a padrões de planejamento e execução: duração estimada e real, horários de conclusão, recomendações aceitas, motivos fornecidos voluntariamente e relação entre energia declarada e execução.

**AI-13.** O sistema não deve inferir características médicas, psicológicas, financeiras, identitárias ou outras categorias sensíveis.

**AI-14.** O usuário deve poder marcar uma recomendação como útil, inadequada, baseada em contexto incorreto ou indesejada como categoria futura de sugestão.

**AI-15.** A comunicação deve ser clara sem representar incerteza como certeza factual.

## 14. Memória do MVP

### 14.1 Conteúdo permitido

- preferências declaradas;
- objetivo semanal atual e histórico necessário;
- horários informados;
- energia declarada;
- recomendações aceitas ou rejeitadas;
- motivos opcionais de adiamento;
- durações estimadas e reais;
- inferências confirmadas.

### 14.2 Estados da memória

1. **Observada:** evento factual registrado.
2. **Proposta:** inferência ainda não confirmada.
3. **Confirmada:** aceita pelo usuário.
4. **Rejeitada:** recusada e não utilizável como fato.
5. **Desatualizada:** pode não representar mais o usuário.
6. **Arquivada:** preservada, mas não usada em recomendações correntes.
7. **Excluída:** removida de acordo com a política de dados.

**MEM-01.** Inferências devem nascer como propostas.

**MEM-02.** O usuário deve poder confirmar, rejeitar, corrigir, arquivar e excluir.

**MEM-03.** Cada memória deve distinguir tipo, origem e atualidade.

**MEM-04.** Memórias rejeitadas permanecem bloqueadas por padrão e só podem ser reconsideradas por solicitação do usuário ou após mudança explícita no fato relacionado.

**MEM-05.** Informações contraditórias devem gerar revisão, não resolução silenciosa.

**MEM-06.** A interface do MVP pode ser simples, mas deve permitir auditoria e controle completos.

**MEM-07.** Exclusão e arquivamento devem possuir significados distintos e visíveis.

**MEM-08.** Histórico pessoal pertence ao usuário e é preservado por padrão; registros técnicos e detalhes transitórios de recomendações podem possuir retenção configurável.

## 15. Taxonomia de ações e consentimentos

| Categoria | Exemplos | Regra no MVP |
|---|---|---|
| Leitura local | Consultar tarefas e compromissos | Sem confirmação adicional |
| Cálculo reversível | Detectar conflito, ordenar proposta | Sem confirmação adicional |
| Inferência | Sugerir padrão de comportamento | Deve ser identificada como hipótese |
| Recomendação | Propor prioridade ou Agora | Não altera dados importantes |
| Rascunho | Preparar plano ou reorganização | Exige aprovação geral para aplicação |
| Alteração estrutural | Mudar prazo, objetivo ou recorrência | Confirmação explícita |
| Cancelamento ou exclusão | Cancelar tarefa, excluir memória | Confirmação e desfazer quando possível |
| Transmissão externa | Enviar dados a serviço de IA | Permissão da categoria e indicação visível |
| Sincronização ou compartilhamento | Enviar dados em segundo plano | Fora do MVP; nunca implícito |

**CNS-01.** O usuário deve poder conceder ou revogar permissão por categoria de dado.

**CNS-02.** Categorias iniciais devem separar tarefas, compromissos, projetos, memória, documentos, ideias e anotações.

**CNS-03.** Uma permissão concedida não autoriza categorias futuras automaticamente.

**CNS-04.** Antes da primeira transmissão de cada categoria, o sistema deve informar serviço, finalidade e tipo de dado.

**CNS-05.** Toda transmissão deve gerar registro consultável.

**CNS-06.** A revogação deve impedir novos envios, sem alegar apagar dados já processados por terceiros quando isso não puder ser garantido.

**CNS-07.** Dados sem permissão não podem ser incluídos indiretamente no contexto enviado.

**CNS-08.** O usuário deve poder marcar um item específico como “nunca enviar”, mesmo quando sua categoria estiver autorizada.

**CNS-09.** Antes de uma operação inédita, o sistema deve tornar visíveis as categorias que pretende utilizar.

## 16. Privacidade, segurança e dados

**PRV-01.** Dados pessoais devem permanecer locais por padrão.

**PRV-02.** Nenhuma sincronização em nuvem deve ser ativada no MVP.

**PRV-03.** O sistema deve informar claramente quando um recurso depende de transmissão externa.

**PRV-04.** O usuário deve visualizar histórico contendo data, serviço, finalidade, categorias e referências aos itens enviados.

**PRV-05.** O usuário deve exportar todos os seus dados em formato recuperável e, quando aplicável, legível.

**PRV-06.** O usuário deve excluir dados e compreender o alcance da exclusão.

**PRV-07.** Diagnósticos permanecem locais por padrão; compartilhamento exige ação consciente.

**PRV-08.** Conteúdo pessoal não deve aparecer em registros técnicos desnecessários.

**PRV-09.** O produto deve permitir evolução dos dados sem perda silenciosa de histórico.

**PRV-10.** Requisitos detalhados de proteção, integridade e recuperação deverão ser definidos na arquitetura sem enfraquecer estas garantias.

**PRV-11.** Dados principais e backups não devem ficar legíveis por acesso casual aos arquivos do dispositivo.

**PRV-12.** Exportação completa e restauração exigem confirmação explícita.

**PRV-13.** A proteção deve considerar a identidade local do usuário no Windows; bloqueio adicional do aplicativo é opcional no MVP.

**PRV-14.** O histórico de transmissão não deve duplicar integralmente conteúdo sensível por padrão.

## 17. Backup, exportação e restauração

**BKP-01.** Backup local faz parte do MVP.

**BKP-02.** O usuário deve criar uma exportação manual completa.

**BKP-03.** O usuário deve configurar backup local periódico em destino escolhido por ele, com frequência padrão diária ajustável.

**BKP-04.** O sistema deve informar quando não existir backup recente, sem gerar pressão excessiva.

**BKP-05.** A restauração deve validar a integridade antes de substituir o estado corrente.

**BKP-06.** Falha de backup não pode ser apresentada como sucesso.

**BKP-07.** O processo deve explicar quais dados estão incluídos.

**BKP-08.** Backup em nuvem e sincronização ficam fora do MVP.

**BKP-09.** O sistema deve preservar múltiplas versões recentes e tornar a política de retenção compreensível e configurável.

**BKP-10.** Antes de restaurar, o sistema deve criar um ponto de recuperação do estado corrente quando houver condições seguras para isso.

**BKP-11.** Backups devem receber proteção equivalente à dos dados principais.

**BKP-12.** Falta de espaço, destino indisponível ou corrupção devem gerar estado de falha explícito.

## 18. Linguagem e personalidade

**LNG-01.** A voz deve ser calma, inteligente, objetiva, educada e breve.

**LNG-02.** O sistema não deve ser dramático, infantil ou excessivamente motivacional.

**LNG-03.** Mensagens não devem utilizar culpa, ameaça, vergonha ou urgência artificial.

**LNG-04.** Diante de tarefa não realizada, o sistema deve investigar somente quando a resposta puder melhorar uma decisão relevante.

**LNG-05.** O usuário deve poder ignorar perguntas opcionais.

**LNG-06.** O sistema não deve se apresentar como humano.

## 19. Direção de experiência

**UX-01.** A interface deve priorizar hierarquia clara, espaço em branco, tipografia confortável e poucas cores.

**UX-02.** A tela inicial não deve parecer um painel cheio de widgets.

**UX-03.** Informações secundárias devem ser reveladas progressivamente.

**UX-04.** Feedback de conclusão deve ser elegante e não gamificado em excesso.

**UX-05.** Fluxos essenciais devem funcionar por teclado.

**UX-06.** Contraste, leitores de tela e redução de movimento devem ser considerados requisitos do MVP.

**UX-07.** O produto deve permitir adiar check-in e planejamento sem penalidade.

**UX-08.** O usuário deve sair de qualquer fluxo conversacional opcional sem perder dados.

## 20. Métricas e plano de validação

### 20.1 Desenho do piloto

- Participante inicial: fundador.
- Linha de base: uma semana antes do uso do MVP.
- Período inicial de avaliação: quatro semanas de uso diário.
- Metas numéricas: definidas após a linha de base.

### 20.2 Métricas principais

**MET-01 — Carga mental percebida.** Avaliação diária curta e reflexão semanal.

**MET-02 — Decisões de organização.** Proxies: tempo de planejamento, planos aceitos sem edição e reorganizações manuais.

**MET-03 — Tempo para iniciar.** Intervalo entre abertura do sistema e início da primeira atividade relevante.

### 20.3 Métricas secundárias

**MET-04.** Clareza percebida ao iniciar o dia.

**MET-05.** Confiança nas recomendações.

**MET-06.** Tarefas importantes esquecidas.

**MET-07.** Recomendações aceitas, ajustadas e rejeitadas.

**MET-08.** Resultado posterior das recomendações aceitas.

**MET-09.** Frequência e causa de replanejamentos.

**MET-10.** Uso do Modo Foco.

**MET-11.** Tempo gasto planejando.

**MET-12.** Retomada após ausência.

**MET-13.** Tempo e esforço percebidos para alimentar e manter o sistema.

**MET-14.** Frequência de abertura espontânea e proporção de dias em que o sistema orientou a primeira atividade.

**MET-15.** Intenção declarada de continuar utilizando o produto após o piloto.

As métricas não devem ser transformadas em placar de valor pessoal. Aderência rígida ao plano não será considerada sinônimo de sucesso.

### 20.4 Critério de validação

Após a linha de base, serão definidos limiares objetivos. A hipótese será sustentada quando evidências quantitativas e qualitativas indicarem, em conjunto:

- menor carga mental percebida;
- redução do esforço de planejamento;
- início mais rápido de atividades relevantes;
- confiança suficiente para uso diário;
- adaptação sem aumento de culpa ou vigilância percebida.

## 21. Critérios de aceitação do ciclo completo

**ACC-01.** Um novo usuário consegue capturar itens e chegar à primeira orientação útil sem cadastrar toda a vida.

**ACC-02.** Compromissos fixos e tarefas flexíveis aparecem sem conflito silencioso.

**ACC-03.** O plano apresenta no máximo três prioridades e permite aprovação única.

**ACC-04.** O **Agora** sempre apresenta uma orientação válida ou comunica falta de contexto.

**ACC-05.** O usuário substitui o **Agora** sem reconstruir obrigatoriamente o plano.

**ACC-06.** Uma mudança relevante produz rascunho explicado e não altera o plano antes da aprovação.

**ACC-07.** O ciclo de captura, planejamento, execução, replanejamento e encerramento funciona sem IA externa.

**ACC-08.** Indisponibilidade de modelo não impede acesso aos dados.

**ACC-09.** Nenhum dado sem permissão é enviado a serviço externo.

**ACC-10.** Toda transmissão autorizada aparece no histórico.

**ACC-11.** Inferências aparecem como hipóteses e podem ser rejeitadas.

**ACC-12.** Backup local pode ser criado e restaurado com validação.

**ACC-13.** Perguntas opcionais podem ser ignoradas sem bloqueio ou culpa.

**ACC-14.** O usuário controla inicialização, notificações, horários silenciosos e proatividade.

**ACC-15.** O piloto consegue coletar linha de base e métricas sem transformar a experiência em vigilância.

**ACC-16.** A experiência principal assistida por IA externa funciona somente após consentimento válido e apresenta limitações quando contexto autorizado estiver incompleto.

**ACC-17.** Projetos do MVP permanecem limitados a nome, descrição opcional, estado e vínculos.

**ACC-18.** Ideias e anotações podem ser convertidas, arquivadas ou excluídas sem criar módulos adicionais.

**ACC-19.** Recorrências semanais permitem editar uma ocorrência ou toda a série sem ambiguidade.

**ACC-20.** Gatilhos não urgentes não interrompem sessão produtiva.

**ACC-21.** Itens marcados como “nunca enviar” não aparecem em transmissões externas.

## 22. Funcionalidades pós-MVP

Entram somente após validação do ciclo principal:

- integração com Google Calendar ou Outlook Calendar;
- projetos completos com progresso, documentos e visualizações;
- hábitos e consistência;
- metas mensais e anuais;
- Pomodoro avançado e modelos personalizados;
- memória avançada e reflexão longitudinal;
- módulo completo de estudos;
- biblioteca de PDFs, vídeos, links e documentos;
- relatórios semanais narrativos avançados;
- modos de Revisão e Descanso mais profundos;
- modelos locais de IA;
- sincronização e backup opcional em nuvem;
- outras plataformas.

O calendário será o primeiro candidato pós-MVP. Antes disso, um teste deverá verificar se o cadastro manual compromete a confiabilidade do ciclo diário. Se comprometer materialmente, ele será reclassificado como MVP condicional.

## 23. Experimentos futuros

Não constituem compromissos de implementação:

- detectar aplicativos abertos no Windows;
- reconhecer projeto ativo no VS Code;
- sugerir registro automático de tempo;
- integrar GitHub, Drive, OneDrive, e-mail, navegador ou Spotify;
- utilizar sinais de sono ou saúde;
- permitir autonomia configurável da IA;
- colaborar com outras pessoas;
- oferecer recursos empresariais;
- expandir para macOS, Linux e dispositivos móveis.

Sinais do computador deverão ser tratados como evidências falíveis, nunca como fatos, e dependerão de consentimento granular.

## 24. Fora de escopo do MVP

- colaboração, equipes e clientes;
- compartilhamento de projetos;
- Kanban completo;
- finanças e relacionamentos;
- gestão documental avançada;
- sincronização em nuvem;
- contexto automático do computador;
- automações complexas;
- múltiplos fornecedores com paridade funcional;
- IA com autorização ampla;
- exigência de vincular toda tarefa a um objetivo;
- gamificação baseada em culpa, punição ou competição.

## 25. Guardrails de escopo

Uma funcionalidade não entra no MVP quando:

- não melhora diretamente captura, planejamento, Agora, execução, adaptação ou encerramento;
- pode ser substituída no piloto por uma interação manual simples;
- adiciona classificação obrigatória sem reduzir esforço posterior;
- exige nova área sem melhorar orientação;
- antecipa colaboração ou escala ainda não validada;
- reduz privacidade ou controle sem necessidade clara;
- não possui critério de aceitação observável;
- torna o sistema tecnicamente sofisticado sem simplificar a vida do usuário.

## 26. Dependências e decisões para arquitetura

A arquitetura deverá responder, sem modificar os requisitos deste PRD:

- como garantir persistência Local First confiável;
- como proteger dados no dispositivo;
- como registrar histórico e permitir migrações;
- como executar backup e restauração seguros;
- como separar lógica determinística e capacidades de IA;
- como aplicar permissões por categoria de dado;
- como auditar transmissões externas;
- como manter a interface funcional sem conexão;
- como suportar futura substituição de fornecedor sem abstração prematura;
- como permitir integrações futuras sem incluí-las no MVP;
- como oferecer notificações e inicialização configuráveis no Windows;
- como coletar métricas locais com privacidade.
- quais versões do Windows serão suportadas no piloto;
- como implementar um único fornecedor externo sem acoplamento irreversível;
- como aplicar o horário operacional e preservar intenção em mudanças de fuso;
- como implementar retenção configurável de registros técnicos;
- como proteger dados e backups contra acesso casual usando o contexto local do Windows.

## 27. Aprovação e controle de mudanças

Este PRD 1.2 está oficialmente aprovado e congelado como contrato de produto para o início da arquitetura.

Durante a arquitetura:

1. decisões técnicas devem respeitar os requisitos e princípios permanentes;
2. itens pós-MVP não devem ser tratados como dependências obrigatórias;
3. refinamentos operacionais sem impacto estratégico podem ser registrados como esclarecimentos;
4. ambiguidades que alterem missão, proposta de valor, escopo significativo ou princípio permanente exigem revisão pontual do PRD;
5. toda mudança aprovada deve receber nova versão e registro de motivo.

Arquitetura, tecnologias, wireframes e implementação permanecem fora deste documento.

Launch a new agent that has access to various tools (the specific list of tools available to the agent is dynamic). When you are searching for a keyword or file and are not confident that you will find the right match on the first try, use the Agent tool to perform the search for you. For example:

When to use the Agent tool:
  - If you are searching for a keyword like "config" or "logger", or for questions like
   "which file does X?", the Agent tool is strongly recommended

When NOT to use the Agent tool:
  - If you want to read a specific file path, use the View or Globtool instead of the
  Agent tool, to find the match more quickly
  - If you are searching for a specific class definition like "class Foo", use the Glob
   tool instead, to find the match more quickly
  - If you are searching for code within a specific file or set of 2-3 files, use the
  View tool instead of the Agent tool, to find the match more quickly
  - Writing code and running bash commands (use other tools for that)
  - Other tasks that are not related to searching for a keyword or file

Usage notes:
1. Launch multiple agents concurrently whenever possible, to maximize performance; to
do that, use a single message with multiple tool uses
2. When the agent is done, it will return a single message back to you. The result
returned by the agent is not visible to the user. To show the user the result, you
should send a text message back to the user with a concise summary of the result.
3. Each agent invocation is stateless. You will not be able to send additional
messages to the agent, nor will the agent be able to communicate with you outside of
its final report. Therefore, your prompt should contain a highly detailed task
description for the agent to perform autonomously and you should specify exactly what
information the agent should return back to you in its final and only message to
you.
4. The agent's outputs should generally be trusted
5. Clearly tell the agent whether you expect it to write code or just to do research
(search, file reads, web fetches, etc.), since it is not aware of the user's intent
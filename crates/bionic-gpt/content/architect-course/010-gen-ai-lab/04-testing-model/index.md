# Test the Platform

Whether you connected an API provider or configured Ollama, you can now test
the complete Bionic environment.

Open the Bionic chat console, select the model you configured, and enter this
prompt:

```text
Use your tools to complete this test:

1. Get the current date and time in UTC.
2. Read https://example.com and summarize it in one sentence.
3. Create /home/user/output/model-test.md with both results.
4. Tell me which tools you used and link to the finished report.
```

This checks that the model can select tools, retrieve current information,
read a URL, create a file in its sandbox, and return a finished artifact.

The screenshot below shows a simpler code-generation check in the same chat
console. It can be replaced with the result of the full test later.

![An example response from Bionic](./test-results.png "Example platform response")

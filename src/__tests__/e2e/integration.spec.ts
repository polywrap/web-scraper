import { PolywrapClient } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";

jest.setTimeout(60000);

describe("Web Scraper Wrapper End to End Tests", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  })

  it("calls get_links", async () => {
    const uri = "https://polywrap.io";

    const result = await client.invoke<App.Module_GetLinks>({
      uri: wrapperUri,
      method: "get_links",
      args: { uri: uri }
    });

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    // Add additional assertions as needed
  });

  it("calls get_text", async () => {
    const url = "https://polywrap.io";

    const result = await client.invoke<App.Module_GetText>({
      uri: wrapperUri,
      method: "get_text",
      args: { url: url }
    });

    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    // Add additional assertions as needed
  });
});

import { PolywrapClient } from "@polywrap/client-js";
import * as App from "../types/wrap";
import path from "path";

jest.setTimeout(60000);

describe("Module Wrapper End to End Tests", () => {

  const client: PolywrapClient = new PolywrapClient();
  let wrapperUri: string;

  beforeAll(() => {
    const dirname: string = path.resolve(__dirname);
    const wrapperPath: string = path.join(dirname, "..", "..", "..");
    wrapperUri = `fs/${wrapperPath}/build`;
  })

  it("calls getText", async () => {
    const expectedUrl: string = "http://polywrap.io";

    const result = await client.invoke<App.Module_GetTextResult>({
      uri: wrapperUri,
      method: "getText",
      args: { url: expectedUrl }
    });
    console.log(result)
    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value.text).toBeDefined();
  });

  it("calls getLinks", async () => {
    const expectedUrl: string = "http://polywrap.io";

    const result = await client.invoke<App.Module_GetLinksResult>({
      uri: wrapperUri,
      method: "getLinks",
      args: { uri: expectedUrl }
    });
    console.log(result)
    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value.links).toBeDefined();
  });
});

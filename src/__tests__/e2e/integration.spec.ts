import { PolywrapClient } from "@polywrap/client-js";

import { WebScraper } from "../types/wrap";

jest.setTimeout(60000);

describe("WebScraper", () => {

  const client = new PolywrapClient();
  const localWrap = `file://${__dirname}/../../../build`

  const webScraper = new WebScraper(client, undefined, localWrap);

  it("get_links", async () => {
    const result = await webScraper.get_links({
      url: "https://webscraper.io/test-sites/e-commerce/allinone"
    });
    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value).toContain(`/
#page-top
/
/cloud-scraper
/pricing
#section3
/documentation
/tutorials
/how-to-videos
/test-sites
https://forum.webscraper.io/
https://chrome.google.com/webstore/detail/web-scraper/jnhgnonknehpejjnehehllkliplmbmhn?hl=en
https://cloud.webscraper.io/
/test-sites/e-commerce/allinone
/test-sites/e-commerce/allinone/phones
/test-sites/e-commerce/allinone/computers`
    );
  });

  it("get_text", async () => {
    const result = await webScraper.get_text({
      url: "https://webscraper.io/test-sites/e-commerce/allinone"
    });
    expect(result.ok).toBeTruthy();
    if (!result.ok) return;
    expect(result.value).toContain(
      `Lenovo ThinkPad L460, 14" FHD IPS, Core i7-6600U, 8GB, 256GB SSD, Windows 10 Pro`
    );
  });
});

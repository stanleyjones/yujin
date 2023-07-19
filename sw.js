import init, { handle_req } from "./pkg/yujin.js";

self.addEventListener("install", () => {
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(clients.claim());
});

self.addEventListener("fetch", (event) => {
  if (event.request.headers.has("X-Many-Request")) {
    event.respondWith(
      (async () => {
        await init();
        const reqBody = await event.request.json();
        const resBody = await handle_req(reqBody);
        return new Response(JSON.stringify(resBody), {
          headers: { "Content-Type": "application/json" },
          statusText: "Intercepted",
        });
      })()
    );
  }
});

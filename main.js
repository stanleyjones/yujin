if ("serviceWorker" in navigator) {
  navigator.serviceWorker
    .register("/sw.js", { type: "module" })
    .catch(console.error);
}

window.addEventListener("submit", async (event) => {
  event.preventDefault();

  const data = new FormData(event.target);
  const url = data.get("url");
  data.delete("url");
  const body = JSON.stringify(Object.fromEntries(data.entries()));

  try {
    const res = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "X-Many-Request": true,
      },
      mode: "cors",
      body,
    });
    const json = await res.json();
    const el = document.getElementById("response");
    el.innerHTML = JSON.stringify(json, null, 2);
  } catch (error) {
    console.log("[MAIN] Fetch error:", error);
  }
});

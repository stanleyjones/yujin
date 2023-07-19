if ("serviceWorker" in navigator) {
  navigator.serviceWorker
    .register("/sw.js", { type: "module" })
    .catch(console.error);
}

window.addEventListener("submit", async (event) => {
  event.preventDefault();
  const data = new FormData(event.target);
  try {
    const res = await postManyRequest(
      data.get("url"),
      JSON.stringify(Object.fromEntries(data.entries()))
    );
    if (res.ok) {
      const json = await res.json();
      setInner("response", JSON.stringify(json, null, 2));
    }
  } catch (e) {
    console.log("Fetch failed", e);
  }
});

function postManyRequest(url, body) {
  return fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-Many-Request": true,
    },
    body,
  });
}

function setInner(id, html) {
  const el = document.getElementById(id);
  el.innerHTML = html;
}

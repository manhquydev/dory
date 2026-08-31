import { parseJournalLines } from "./journal.js";

const LABELS = {
  "session/open": "Mở phiên",
  "journal/note": "Ghi chú",
  "session/goal": "Mục tiêu",
  "flow/invoke": "Chạy Flow",
  "flow/result": "Kết quả Flow",
  _broken: "Dòng hỏng",
};

export function escapeHtml(text) {
  return String(text)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

function eventLabel(type) {
  if (Object.hasOwn(LABELS, type)) return LABELS[type];
  return `Sự kiện ${type || "unknown"}`;
}

function cardBody(event) {
  const parts = [];
  if (typeof event.text === "string") {
    parts.push(`<p>${escapeHtml(event.text)}</p>`);
  }
  if (typeof event.workspace === "string") {
    parts.push(`<p><code>${escapeHtml(event.workspace)}</code></p>`);
  }
  if (typeof event.code === "string" || typeof event.code === "number") {
    parts.push(`<p><code>${escapeHtml(event.code)}</code></p>`);
  }
  if (typeof event.bin === "string") {
    parts.push(`<p><code>${escapeHtml(event.bin)}</code></p>`);
  }
  if (Array.isArray(event.args)) {
    parts.push(`<p><code>${escapeHtml(event.args.map(String).join(" "))}</code></p>`);
  }
  if (typeof event.stdout === "string" && event.stdout) {
    parts.push(`<p>${escapeHtml(event.stdout)}</p>`);
  }
  if (typeof event.stderr === "string" && event.stderr) {
    parts.push(`<p>${escapeHtml(event.stderr)}</p>`);
  }
  if (typeof event.error === "string" && event.error) {
    parts.push(`<p>${escapeHtml(event.error)}</p>`);
  }
  if (event.type === "_broken" && typeof event.raw === "string") {
    parts.push(`<p>${escapeHtml(event.raw)}</p>`);
  }
  return parts.join("");
}

function renderCards(bytes) {
  const items = parseJournalLines(bytes).map((event) => {
    const type = typeof event.type === "string" ? event.type : "_broken";
    return `<li data-type="${escapeHtml(type)}"><p class="kind">${escapeHtml(eventLabel(type))}</p>${cardBody(event)}</li>`;
  });
  return items.join("");
}

function renderFlowSection(flowPreview, workspace) {
  const cwd = escapeHtml(workspace);
  if (!flowPreview || flowPreview.ok === false) {
    const reason = escapeHtml(flowPreview?.error || "không chạy được");
    return `<section aria-labelledby="flow-h">
      <h2 id="flow-h">Chạy Flow</h2>
      <p id="flow-preview">từ chối: ${reason}</p>
    </section>`;
  }
  const bin = escapeHtml(flowPreview.bin || "flow.sh");
  return `<section aria-labelledby="flow-h">
      <h2 id="flow-h">Chạy Flow</h2>
      <p id="flow-preview">Sẽ chạy: <code>${bin}</code> status<br>Thư mục: <code>${cwd}</code></p>
      <form id="flow">
        <label class="check"><input type="checkbox" name="confirm" value="true" required> Tôi xác nhận chạy lệnh trên</label>
        <button type="submit">Chạy Flow</button>
      </form>
    </section>`;
}

const PAGE_CSS = `* { box-sizing: border-box; }
body {
  margin: 0 auto; padding: 1rem; max-width: 40rem;
  color: #1a1a1a; background: #f4f4f0;
  font: 1rem/1.55 system-ui, "Segoe UI", "Noto Sans", sans-serif;
}
h1, h2 { font-size: 1.25rem; line-height: 1.3; }
.law, #journal, #workspace, footer { overflow-wrap: anywhere; }
#journal { list-style: none; margin: 0 0 1.5rem; padding: 0; }
#journal li {
  margin: 0 0 0.75rem; padding: 0.75rem;
  border: 1px solid #3d3d3d; background: #fafaf6;
}
.kind { font-weight: 700; margin: 0 0 0.25rem; }
code { font-family: inherit; }
label, button { display: block; }
textarea, button {
  width: 100%; min-height: 2.75rem; margin: 0.5rem 0;
  padding: 0.5rem; font: inherit; color: inherit;
  background: #fafaf6; border: 1px solid #3d3d3d;
}
textarea { min-height: 6rem; }
button { cursor: pointer; font-weight: 700; border-width: 2px; }
button:hover { background: #1a1a1a; color: #f4f4f0; }
:focus-visible { outline: 3px solid #1a1a1a; outline-offset: 2px; }
.check {
  display: flex; align-items: center; gap: 0.5rem;
  min-height: 2.75rem; cursor: pointer;
}
.check input { width: 1.25rem; height: 1.25rem; }
footer { margin-top: 2rem; padding-top: 1rem; border-top: 1px solid #3d3d3d; }
@media (min-width: 1024px) { body { padding: 2rem; } }`;

export function renderJournalPage(bytes, { workspace, flowPreview } = {}) {
  const path = escapeHtml(workspace || "");
  return `<!DOCTYPE html>
<html lang="vi">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Nhật ký phiên</title>
  <style>${PAGE_CSS}</style>
</head>
<body>
  <header>
    <h1>Nhật ký phiên</h1>
    <p class="law" translate="no">Log projection of <code>.dory/journal.jsonl</code> on 127.0.0.1:7380. Not a workplace. Not a pane. Not a terminal.</p>
    <p>Thư mục phiên <code id="workspace">${path}</code></p>
  </header>
  <main>
    <ol id="journal">${renderCards(bytes)}</ol>
    <form id="goal" action="/goal" method="post">
      <label for="goal-text">Mục tiêu</label>
      <textarea id="goal-text" name="text" required placeholder="Bạn muốn làm gì?"></textarea>
      <button type="submit">Ghi mục tiêu</button>
    </form>
    ${renderFlowSection(flowPreview, workspace || "")}
  </main>
  <footer>Lamp: <code>npx @manhquy/dory@0.1.0-next.2 dory-serve -- serve --workspace ${path}</code> then open this page. Not the desk. Not an icon.</footer>
  <script>
(function () {
  var form = document.getElementById("flow");
  if (!form) return;
  form.addEventListener("submit", function (e) {
    e.preventDefault();
    var box = form.querySelector('input[name="confirm"]');
    if (!box || !box.checked) return;
    fetch("/flow", {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ confirm: true })
    }).then(function () { location.assign("/"); });
  });
})();
  </script>
</body>
</html>
`;
}

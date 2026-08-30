function sendJson(res, status, obj) {
  const buf = Buffer.from(`${JSON.stringify(obj)}\n`);
  res.writeHead(status, {
    "content-type": "application/json; charset=utf-8",
    "content-length": buf.length,
  });
  res.end(buf);
}

function insideWorkplace(req) {
  return req.headers["x-dory-inside"] === "1";
}

export async function handleWorkplace(req, res, url, workplace, readJsonBody) {
  try {
    if (req.method === "POST" && url.pathname === "/workplace/open") {
      sendJson(res, 200, { ok: true, workplace: workplace.open() });
      return true;
    }
    if (req.method === "GET" && url.pathname === "/workplace") {
      sendJson(res, 200, { ok: true, workplace: workplace.get() });
      return true;
    }
    if (req.method === "POST" && url.pathname === "/workplace/detach") {
      sendJson(res, 200, { ok: true, workplace: workplace.detach() });
      return true;
    }
    if (req.method === "POST" && url.pathname === "/workplace/split") {
      sendJson(res, 200, { ok: true, workplace: workplace.split() });
      return true;
    }
    if (req.method === "POST" && url.pathname === "/workplace/focus") {
      const body = await readJsonBody(req);
      sendJson(res, 200, { ok: true, workplace: workplace.focus(body.pane_id) });
      return true;
    }
    if (req.method === "POST" && url.pathname === "/workplace/coordinate") {
      const body = await readJsonBody(req);
      const result = await workplace.coordinate({
        from: body.from,
        to: body.to,
        text: body.text,
        inside: insideWorkplace(req),
      });
      sendJson(res, 200, result);
      return true;
    }
    const attach = url.pathname.match(/^\/workplace\/panes\/([^/]+)\/session$/);
    if (req.method === "POST" && attach) {
      const result = await workplace.attachSession(
        decodeURIComponent(attach[1]),
      );
      sendJson(res, 200, result);
      return true;
    }
    const note = url.pathname.match(
      /^\/workplace\/panes\/([^/]+)\/session\/note$/,
    );
    if (req.method === "POST" && note) {
      const body = await readJsonBody(req);
      const result = await workplace.sessionNote(
        decodeURIComponent(note[1]),
        body.text,
      );
      sendJson(res, 200, result);
      return true;
    }
    const input = url.pathname.match(/^\/workplace\/panes\/([^/]+)\/input$/);
    if (req.method === "POST" && input) {
      const body = await readJsonBody(req);
      sendJson(
        res,
        200,
        workplace.input(decodeURIComponent(input[1]), body.data),
      );
      return true;
    }
    const pane = url.pathname.match(/^\/workplace\/panes\/([^/]+)$/);
    if (req.method === "GET" && pane) {
      sendJson(res, 200, {
        ok: true,
        pane: workplace.pane(decodeURIComponent(pane[1])),
      });
      return true;
    }
    return false;
  } catch (err) {
    sendJson(res, err.statusCode || 500, { ok: false, error: err.message });
    return true;
  }
}

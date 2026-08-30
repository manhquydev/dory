export function createIds() {
  let windows = 0;
  let tabs = 0;
  let panes = 0;
  let sessions = 0;
  const retired = new Set();

  function mint(id) {
    if (retired.has(id)) {
      throw new Error(`dory: refused to reuse retired id ${id}`);
    }
    return id;
  }

  return {
    window() {
      windows += 1;
      return mint(`w${windows}`);
    },
    tab(windowId) {
      tabs += 1;
      return mint(`${windowId}:t${tabs}`);
    },
    pane(windowId) {
      panes += 1;
      return mint(`${windowId}:p${panes}`);
    },
    session() {
      sessions += 1;
      return mint(`s${sessions}`);
    },
    retire(id) {
      retired.add(id);
    },
    retired() {
      return new Set(retired);
    },
  };
}

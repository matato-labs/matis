function nested() {
  try { try { return <div />; } catch { return null; } }
  catch { return <span />; }
}
function flat() {
  try { return <div />; } catch { return null; }
}
function independent() {
  try {
    function inner() { try { return <div />; } catch { return null; } }
    return inner();
  } catch { return null; }
}

function trim(input) {
  return input.trim().replace(/^\s+/gm, "");
}

function trimmed(input) {
  return trim(input.join("\n"));
}

exports.trim = trim;
exports.trimmed = trimmed;

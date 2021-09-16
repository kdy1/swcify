export function trim(input: string) {
  return input.trim().replace(/^\s+/gm, '');
}

export function trimmed(input: TemplateStringsArray) {
  return trim(input.join('\n'));
}

import React from "react";

export function Colored({
  text,
  colorCodes,
}: {
  text: string;
  colorCodes: Record<string, string>;
}) {
  const parts: JSX.Element[] = [];
  let hadCodeSymbol = false;
  let currentColor = "000000";
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === "&") {
      hadCodeSymbol = true;
      continue;
    }
    if (hadCodeSymbol) {
      hadCodeSymbol = false;
      const color = colorCodes[c];
      if (color) {
        currentColor = color;
        continue;
      }
    }

    parts.push(
      <span style={{ fontWeight: "bold", color: `#${currentColor}` }}>{c}</span>
    );
  }

  return <div>{parts}</div>;
}

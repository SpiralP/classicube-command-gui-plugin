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
  text.split("").forEach((c) => {
    if (c === "&") {
      hadCodeSymbol = true;
      return;
    }
    if (hadCodeSymbol) {
      hadCodeSymbol = false;
      const color = colorCodes[c];
      if (color) {
        currentColor = color;
        return;
      }
    }

    parts.push(
      <span style={{ fontWeight: "bold", color: `#${currentColor}` }}>{c}</span>
    );
  });

  return <div>{parts}</div>;
}

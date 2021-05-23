import React, { useContext, useEffect, useRef, useState } from "react";
import { ConnectionContext } from "../Connection";
import { JsonEvent } from "../types";

export function RenderedText({
  size = 16,
  shadow = false,
  children,
}: {
  size?: number;
  shadow?: boolean;
  children: string;
}) {
  const connection = useContext(ConnectionContext);
  const ref = useRef<HTMLCanvasElement>(null);
  const [rendered, setRendered] = useState(false);

  useEffect(() => {
    if (!connection) return undefined;

    const text = children.startsWith("&") ? children : `&0${children}`;
    function listener(obj: JsonEvent) {
      if (obj.type === "renderedText") {
        (async () => {
          const {
            text: text2,
            size: size2,
            shadow: shadow2,
            pixels,
            width,
            height,
          } = obj.data;
          if (text !== text2 || size !== size2 || shadow !== shadow2) return;

          const bitmap = await createImageBitmap(
            new ImageData(new Uint8ClampedArray(pixels), width, height)
          );

          const canvas = ref.current;
          if (!canvas) throw new Error("!canvas");
          const context = canvas.getContext("bitmaprenderer");
          if (!context) throw new Error("!context");
          context.transferFromImageBitmap(bitmap);
          canvas.width = width;
          canvas.height = height;

          setRendered(true);
        })();
      }
    }
    connection.addListener(listener);
    connection.send({ type: "renderText", data: { text, size, shadow } });

    return () => {
      connection.removeListener(listener);
    };
  }, [children, connection, shadow, size]);

  if (!connection) {
    return <span>{children}</span>;
  }
  return (
    <span>
      {rendered ? null : children}
      <canvas ref={ref} />
    </span>
  );
}

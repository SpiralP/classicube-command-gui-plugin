import React, { useEffect, useRef } from "react";
import { Connection } from "../Connection";

export function RenderedText({
  children,
  connection,
}: {
  children: string;
  connection: Connection;
}) {
  const ref = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    function listener(obj: JsonEvent) {
      if (obj.type === "renderedText") {
        (async () => {
          const { text, pixels, width, height } = obj.data;
          if (text !== children) return;

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
        })();
      }
    }
    connection.addListener(listener);
    connection.send({ type: "renderText", data: children });

    return () => {
      connection.removeListener(listener);
    };
  }, []);

  return <canvas ref={ref} />;
}

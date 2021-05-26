import memoizee from "memoizee";
import React, { useContext, useEffect, useRef, useState } from "react";
import { useInView } from "react-intersection-observer";
import { Connection, ConnectionContext } from "../Connection";
import { JsonEvent, RenderTextJsonMessage } from "../types";

const renderText = memoizee(
  async (
    { text, size, shadow }: RenderTextJsonMessage["data"],
    connection: Connection
  ): Promise<ImageData> => {
    return new Promise((resolve) => {
      function listener(obj: JsonEvent) {
        if (
          obj.type === "renderedText" &&
          obj.data.text === text &&
          obj.data.size === size &&
          obj.data.shadow === shadow
        ) {
          const { pixels, width, height } = obj.data;
          connection.removeListener(listener);

          const imageData = new ImageData(
            new Uint8ClampedArray(pixels),
            width,
            height
          );

          resolve(imageData);
        }
      }
      connection.addListener(listener);
      connection.send({ type: "renderText", data: { text, size, shadow } });
    });
  },
  {
    promise: true,
    normalizer(args) {
      return JSON.stringify(args[0]);
    },
  }
);

export function RenderedText({
  size = 16,
  shadow = true,
  children,
}: {
  size?: number;
  shadow?: boolean;
  children: string;
}) {
  const connection = useContext(ConnectionContext);
  const ref = useRef<HTMLCanvasElement>(null);
  const [rendered, setRendered] = useState(false);
  const { ref: containerRef, inView } = useInView();

  useEffect(() => {
    if (rendered || !connection || !inView) return undefined;

    const text = children.startsWith("&") ? children : `&0${children}`;

    let cancel = false;

    (async () => {
      const imageData = await renderText({ text, size, shadow }, connection);
      if (cancel) return;
      const bitmap = await createImageBitmap(imageData);
      if (cancel) return;

      const canvas = ref.current;
      if (!canvas) throw new Error("!canvas");
      const context = canvas.getContext("bitmaprenderer");
      if (!context) throw new Error("!context");
      context.transferFromImageBitmap(bitmap);
      canvas.width = imageData.width;
      canvas.height = imageData.height;

      setRendered(true);
    })();

    return () => {
      cancel = true;
    };
  }, [children, connection, inView, rendered, shadow, size]);

  return (
    <div ref={containerRef}>
      {rendered ? null : <span>{children}</span>}
      <canvas
        ref={ref}
        style={
          rendered
            ? undefined
            : {
                display: "none",
              }
        }
      />
    </div>
  );
}

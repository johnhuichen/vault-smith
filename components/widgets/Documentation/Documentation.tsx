"use client";

import { useEffect, useMemo, useState } from "react";

import { faCircleQuestion } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import cx from "classnames";
import { debounce } from "lodash";

import useClickOutside from "@/components/hooks/useClickOutside";
import { accentClasses } from "@/components/lib/cssClasses";

import { glossary } from "./glossary";

type DocumentationProps = {
  label: string;
  showLabel?: boolean;
  className?: string;
};

const Documentation = ({
  label,
  showLabel = false,
  className = "",
}: DocumentationProps) => {
  const [showDocumentation, setShowDocumentation] = useState(false);
  const [nodeLeft, setNodeLeft] = useState(0);
  const [nodeRight, setNodeRight] = useState(0);
  const { nodeRef } = useClickOutside({
    handleClickOutside: () => setShowDocumentation(false),
  });

  const definition = useMemo(() => {
    const key = label.toLowerCase();
    if (key in glossary) {
      return glossary[key];
    }
    return `The documents on ${label} is not found.`;
  }, [label]);

  const updateWidths = useMemo(
    () =>
      debounce((entries: ResizeObserverEntry[]) => {
        if (nodeRef.current) {
          const viewportW = entries[0].contentRect.width;
          const tooltipW = Math.min(viewportW - 40, 500);
          const { x, width } = nodeRef.current.getBoundingClientRect();
          if (x - tooltipW < 20) {
            setNodeLeft(20 - x);
            setNodeRight(x - 10 - tooltipW + width / 2);
            return;
          }
          setNodeLeft(-tooltipW + width);
          setNodeRight(0);
        }
      }, 250),
    [nodeRef],
  );

  useEffect(() => {
    const resizeObserver = new ResizeObserver(updateWidths);
    resizeObserver.observe(document.body);
    return () => resizeObserver.disconnect();
  }, [updateWidths]);

  return (
    <>
      {showLabel && label}
      <div
        ref={nodeRef}
        className={cx("relative inline-block ml-1", className)}
      >
        <button onClick={() => setShowDocumentation(!showDocumentation)}>
          <FontAwesomeIcon icon={faCircleQuestion} className={accentClasses} />
        </button>
        {showDocumentation && (
          <div
            className={cx(
              "absolute top-[calc(100%+10px)] z-30",
              "bg-slate-50 p-4 border shadow rounded overflow-y-scroll",
              "text-base text-slate-700 font-normal text-left",
            )}
            style={{ left: nodeLeft, right: nodeRight }}
          >
            {definition}
          </div>
        )}
      </div>
    </>
  );
};

export default Documentation;

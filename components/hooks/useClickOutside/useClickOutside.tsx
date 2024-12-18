import { createRef, useEffect } from "react";

interface Props {
  handleClickOutside: () => void;
}

interface Result {
  nodeRef: React.RefObject<HTMLDivElement>;
}

const useClickOutside = ({ handleClickOutside }: Props): Result => {
  const nodeRef = createRef<HTMLDivElement>();

  useEffect(() => {
    function handleMousedown(this: Document, event: MouseEvent): void {
      if (!nodeRef.current?.contains(event.target as Node)) {
        handleClickOutside();
      }
    }

    document.addEventListener("mousedown", handleMousedown);
    return () => {
      document.removeEventListener("mousedown", handleMousedown);
    };
  }, [nodeRef, handleClickOutside]);

  return { nodeRef };
};

export default useClickOutside;

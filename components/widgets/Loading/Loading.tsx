import cx from "classnames";

interface LoadingProps {
  className?: string;
}

const Loading: React.FC<LoadingProps> = ({ className }: LoadingProps) => {
  return (
    <div
      className={cx("flex items-center justify-center w-10 h-10", className)}
    >
      <div
        className={cx(
          "w-8 h-8 m-1",
          "border-2 border-solid rounded-[50%]",
          "border-t-slate-300 border-r-slate-300 border-b-slate-300 border-l-transparent",
          "animate-[loading_1s_cubic-bezier(0.5,0,0.5,1)_infinite]",
        )}
      />
    </div>
  );
};

export default Loading;

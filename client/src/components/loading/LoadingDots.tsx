import "./LoadingDots.css";

type LoadingDotsProps = {
  visible: boolean;
};

export default function LoadingDots({ visible }: LoadingDotsProps) {
  if (!visible) return null;

  return (
    <div id="loading-dots" className="loading-dots">
      <span />
      <span />
      <span />
    </div>
  );
}

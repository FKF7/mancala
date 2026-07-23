import "./ProcessButton.css";

type ProcessButtonProps = {
  executing: boolean;
  startFunction: () => Promise<void>;
  interrupting: boolean;
  requestInterrupt: () => void;
  startText: string;
  disabled?: boolean;
};

export default function ProcessButton({
  executing,
  startFunction,
  interrupting,
  requestInterrupt,
  startText,
  disabled = false,
}: ProcessButtonProps) {
  return (
    <button
      className={`process-button ${executing ? (interrupting ? "process-button-stopping" : "process-button-running") : "process-button-idle"}`}
      onClick={executing ? requestInterrupt : () => void startFunction()}
      disabled={executing ? interrupting : disabled}
    >
      {executing ? (interrupting ? "Stopping..." : "Stop") : startText}
    </button>
  );
}

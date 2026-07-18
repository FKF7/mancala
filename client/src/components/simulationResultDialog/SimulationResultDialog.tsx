import "./SimulationResultDialog.css";
import { SimulationResultStatus } from "../../enums/mancala.enums";

export type SimulationResultDialogProps = {
  open: boolean;
  status: SimulationResultStatus;
  message: string;
  onClose: () => void;
};

export default function SimulationResultDialog({
  open,
  status,
  message,
  onClose,
}: SimulationResultDialogProps) {
  if (!open) return null;

  return (
    <div className="simulation-dialog-backdrop" onClick={onClose}>
      <div
        className={`simulation-dialog simulation-dialog-${status === SimulationResultStatus.SUCCESS ? "success" : "error"}`}
        onClick={(e) => e.stopPropagation()}
      >
        <h2>{status === SimulationResultStatus.SUCCESS ? "Simulation completed" : "Simulation error"}</h2>

        <p>{message}</p>

        <button onClick={onClose}>OK</button>
      </div>
    </div>
  );
}
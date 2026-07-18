import { ReactNode } from "react";

type FlagButtonProps = {
    contentTrue:  ReactNode;
    contentFalse: ReactNode;
    flag: boolean;
    setFlag: React.Dispatch<React.SetStateAction<boolean>>;
    disabled?: boolean;
}

export default function FlagButton({ contentTrue, contentFalse, flag, setFlag, disabled }: FlagButtonProps) {

    const content = flag ? contentTrue : contentFalse;
    return (
        <button onClick={() => setFlag(!flag)} disabled={disabled}>
            {content}
        </button>
    );
}

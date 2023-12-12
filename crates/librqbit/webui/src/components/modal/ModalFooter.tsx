import { ReactNode } from "react";

export const ModalFooter = ({ children }: { children: ReactNode }) => {
  return <div className="p-3">{children}</div>;
};

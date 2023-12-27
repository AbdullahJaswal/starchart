import { ReactNode } from "react";

export default function MainSidebar({ children }: { children: ReactNode }) {
  return (
    <div className="drawer drawer-open">
      <input id="main-sidebar" type="checkbox" className="drawer-toggle" />
      <div className="drawer-content flex flex-col items-center justify-center">
        {children}
      </div>

      {/*<div className="drawer-side border-r border-base-content">
        <ul className="menu gap-2 p-2 w-32 h-full !text-base-content underline">
          <li>
            <a className="!bg-transparent hover:scale-105">Search</a>
          </li>
          <li>
            <a className="!bg-transparent hover:scale-105">About</a>
          </li>
        </ul>
      </div>*/}
    </div>
  );
}

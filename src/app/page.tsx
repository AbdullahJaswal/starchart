"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type CelestialObject = {
  cartesian: {
    x: number;
    y: number;
    z: number;
  };
  color: string;
  temperature: number;
};

export default function Home() {
  const [stars, setStars] = useState<CelestialObject[]>([]);

  useEffect(() => {
    invoke<string>("stars")
      .then((result: string) =>
        setStars(JSON.parse(result) as CelestialObject[]),
      )
      .catch(console.error);
  }, []);

  return (
    <main>
      <div className="overflow-x-auto">
        <table className="table table-sm table-pin-rows">
          <thead>
            <tr>
              <th></th>
              <th>Cartesian Coordinates</th>
              <th>Color</th>
              <th>Temperature (K)</th>
            </tr>
          </thead>
          <tbody>
            {stars.map((star, index) => {
              const i = index + 1;
              const block_color = `text-[${star.color}]`;

              return (
                <tr key={i}>
                  <td>{i}</td>
                  <td>
                    x: {star.cartesian.x}
                    <br />
                    y: {star.cartesian.y}
                    <br />
                    z: {star.cartesian.z}
                  </td>
                  <td className={`${block_color}`}>{star.color}</td>
                  <td>{star.temperature} K</td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </main>
  );
}

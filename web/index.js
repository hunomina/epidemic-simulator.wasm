import * as wasm from "epidemic-simulator";
import p5 from 'p5';
import { getSketch } from "./sketch";

let simulation = wasm.init_simulation(
    new wasm.Configuration(
        new wasm.Size(40, 40),
        new wasm.SubjectsRepartition(20, 5),
        new wasm.Rules(1, 2)
    ),
    100
);

new p5(getSketch(simulation, {
    fps: 20,
    size: {
        width: 800,
        height: 800
    }
}));
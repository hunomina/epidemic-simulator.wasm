import * as wasm from "epidemic-simulator";
import p5 from 'p5';
import { getSketch } from "./sketch";

function getSimulationModel() {
    return wasm.init_simulation(
        new wasm.Configuration(
            new wasm.Size(
                get_input_value('simulator_height_input'),
                get_input_value('simulator_width_input'),
            ),
            new wasm.SubjectsRepartition(
                get_input_value('sick_percentage_input'),
                get_input_value('recovered_percentage_input'),
            ),
            new wasm.Rules(
                get_input_value('movement_speed_input'),
                get_input_value('safe_distance_input', 'float'),
            )
        ),
        get_input_value('subject_count_input')
    );
}

function init_simulation(simulationModel) {
    return new p5(
        getSketch(
            simulationModel,
            {
                fps: 20,
                size: {
                    width: 800,
                    height: 800
                }
            }
        ),
        'simulationCanvas'
    );
}

/**
 * @param {Event} event 
 */
function resetSimulation(event) {
    event.preventDefault();
    simulation.remove();
    simulation = init_simulation(getSimulationModel());
}

function get_input_value(id, type = 'int') {
    const value = document.getElementById(id).value;
    if (value === '') {
        return null;
    }
    switch (type) {
        case 'int':
            return parseInt(value);
        case 'float':
            return parseFloat(value);
        default:
            return value;
    }
}

var simulation;
(() => {
    document.getElementById('settingsForm').addEventListener('submit', resetSimulation);
    simulation = init_simulation(getSimulationModel());
})();



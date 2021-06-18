import * as wasm from "epidemic-simulator";

/**
 * @param {*} simulation 
 * @param {SketchOptions} options 
 */
export function getSketch(simulation, options) {
    return (sketch) => {

        const SUBJECT_CARACTERISTICS = {
            width: options.size.width / simulation.configuration.size[0],
            height: options.size.height / simulation.configuration.size[1],
        };

        sketch.setup = () => {
            sketch.createCanvas(options.size.width, options.size.height);
            sketch.frameRate(options.fps);
        }

        sketch.draw = () => {
            sketch.clear();
            for (const subject of simulation.subjects) {
                drawSubject(sketch, subject);
            }
            simulation = wasm.next_generation(simulation);
        }

        const drawSubject = (sketch, subject) => {
            const subjectState = subject.state_history[subject.state_history.length - 1].state;
            switch (subjectState) {
                case wasm.State[wasm.State.Healthy]:
                    drawHealthySubject(sketch, subject);
                    break;
                case wasm.State[wasm.State.Sick]:
                    drawSickSubject(sketch, subject);
                    break;
                case wasm.State[wasm.State.Recovered]:
                    drawRecoveredSubject(sketch, subject);
                    break;
            }
        };

        const drawSickSubject = (sketch, subject) => {
            const color = sketch.color(255, 0, 0, 200);
            drawPoint(subject.position, color, color);
        };

        const drawHealthySubject = (sketch, subject) => {
            drawPoint(subject.position, sketch.color(0, 255, 0, 200));
        };

        const drawRecoveredSubject = (sketch, subject) => {
            const color = sketch.color(200, 200);
            drawPoint(subject.position, color, color);
        };

        const drawPoint = ({ x, y }, color, innerColor = null) => {
            if (innerColor) {
                sketch.ellipseMode(sketch.RADIUS);
                sketch.fill(0, 0, 0, 0);
                sketch.stroke(innerColor);
                sketch.ellipse(
                    x * SUBJECT_CARACTERISTICS.width + SUBJECT_CARACTERISTICS.width / 2,
                    y * SUBJECT_CARACTERISTICS.height + SUBJECT_CARACTERISTICS.height / 2,
                    SUBJECT_CARACTERISTICS.width / 2,
                    SUBJECT_CARACTERISTICS.height / 2
                ); // Outer white ellipse
            }

            sketch.ellipseMode(innerColor ? sketch.CENTER : sketch.RADIUS);
            sketch.noStroke();
            sketch.fill(color);
            sketch.ellipse(
                x * SUBJECT_CARACTERISTICS.width + SUBJECT_CARACTERISTICS.width / 2,
                y * SUBJECT_CARACTERISTICS.height + SUBJECT_CARACTERISTICS.height / 2,
                SUBJECT_CARACTERISTICS.width / 2,
                SUBJECT_CARACTERISTICS.height / 2
            );
        }
    };
}

class SketchOptions {
    /**
     * @param {number} fps 
     * @param {Size} size 
     */
    constructor(fps, size) {
        this.fps = fps;
        this.size = size;
    }
}

class Size {
    /**
     * @param {number} width 
     * @param {number} height 
     */
    constructor(width, height) {
        this.width = width;
        this.height = height;
    }
}
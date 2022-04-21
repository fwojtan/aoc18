import subprocess
import sys
from matplotlib import pyplot as plt
import numpy as np
import time

subprocess.call(["cargo",  "build",  "--release"])
day = sys.argv[1]
options = sys.argv[2:]

if len(sys.argv) > 12:
    print("Use fewer options!")

print(options)

colors_list = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf"]

iterations = 500

data = {}
averages = {}
stds_upper = {}
stds_lower = {}
colors = {}

for i, option in enumerate(options):
    data[option] = np.zeros(iterations)
    averages[option] = np.zeros(iterations)
    stds_upper[option] = np.zeros(iterations)
    stds_lower[option] = np.zeros(iterations)
    colors[option] = colors_list[i]
plt.ion()

fig, ax = plt.subplots(figsize=(10, 8))



xs = np.arange(500)

for i in xs:
    ax.cla()
    for option in options:
        output = subprocess.check_output(["./target/release/template",  day,  option])
        output = output.decode().split("\n")
        data[option][i] = float(output[3].replace("ms", ""))
        mean = np.mean(data[option][:i+1])
        averages[option][i] = mean
        std = np.std(data[option][:i+1])
        stds_upper[option][i] = mean+std
        stds_lower[option][i] = mean-std

        ax.set_xlim(0, len(xs[:i+1]))
        values = []
        for key in data:
            values.append(data[key])
            values.append(averages[key])
            values.append(stds_upper[key])
        ax.set_ylim(0, np.max(values)+1)
        ax.fill_between(xs[:i+1], stds_lower[option][:i+1], stds_upper[option][:i+1], alpha=0.15, color=colors[option])        
        ax.plot(xs[:i+1], data[option][:i+1], color=colors[option], label=option, alpha=0.6)
        ax.plot(xs[:i+1], averages[option][:i+1], ls="--", color=colors[option])
    
    ax.legend()
    plt.title("Timing Data")
    plt.xlabel("Iteration")
    plt.ylabel("Time Taken (ms)")
    fig.canvas.draw()
    fig.canvas.flush_events()

plt.show()
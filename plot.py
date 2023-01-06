from pymongo import MongoClient
import matplotlib.pyplot as plt
import argparse

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', type=str, default='"Tromsø"')

    args = parser.parse_args()

    client = MongoClient()
    db = client['weatherlogger']
    collection = db['weatherlog']

    data = list(collection.find({"name": args.name}))

    xs = [d['dt'] for d in data]
    ys = [d['temp'] for d in data]
    ys1 = [d['temp_min'] for d in data]
    ys2 = [d['temp_max'] for d in data]
    ys3 = [d['feels_like'] for d in data]

    plt.scatter(xs, ys, label='temp')
    plt.scatter(xs, ys1, label='temp_min')
    plt.scatter(xs, ys2, label='temp_max')
    plt.scatter(xs, ys3, label='feels_like')
    plt.legend()
    plt.show()

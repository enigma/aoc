#!/usr/bin/env bb

(ns y2025.d07
  (:require [clojure.string :as str]
            [clojure.set :refer [intersection union difference]]))


(defn parse [s]
  (let [[h & rs] (str/split-lines s)
        start (str/index-of h \S)
        splits (map #(set (keep-indexed (fn [i c] (when (= c \^) i)) %)) rs)]
    [start splits]))

(defn part1 [[start splitters]]
  (->> splitters
       (reduce
        (fn [[result beams] layer]
          (let [hits (intersection layer beams)]
            [(+ result (count hits))
             (-> beams
                 (difference hits)
                 (union (into #{} (mapcat (juxt dec inc) hits))))]))
        [0 #{start}])
       first))

(defn part2 [[start splitters]]
  (->> splitters
       (reduce
        (fn [ways layer]
          (->> layer
               (intersection (set (keys ways)))
               (select-keys ways)
               (reduce-kv
                (fn [m pos count]
                  (-> m
                      (dissoc pos)
                      (update (dec pos) (fnil + 0) count)
                      (update (inc pos) (fnil + 0) count)))
                ways)))
        {start 1})
       vals
       (apply +)))

(let [input (->> *command-line-args* last slurp parse)
      res {:part1 (part1 input)
           :part2 (part2 input)}]
  (prn res))
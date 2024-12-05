#!/usr/bin/env bb

(ns y2024.d02
  (:require [babashka.cli :as cli]
            [clojure.string :as str]))


(def cli-options {:input {:default "../../inputs/2024/02.input" :type :string}})

(def opts (cli/parse-opts *command-line-args* {:spec cli-options}))

(def input
  (->> (slurp (get opts :input))
       (str/split-lines)
       (map #(str/split % #" "))
       (map #(map parse-long %))
       (map #(into [] %))))

(defn one-way-safe? [xs]
  (let [pairs (partition 2 1 xs)]
    (every? (fn [[lhs rhs]]
              (< lhs rhs (+ lhs 4)))
            pairs)))

(defn safe? [xs]
  (or (one-way-safe? xs)
      (one-way-safe? (reverse xs))))

(defn part1 [data]
  (count (filter safe? data)))

(defn lenient-safe? [xs]
  (or (safe? xs)
      (let [n (count xs)]
        (->> (range n)
             (map #(into [] (concat (take % xs) (drop (inc %) xs))))
             (some safe?)))))

(defn part2 [data]
  (count (filter lenient-safe? data)))

(prn {:part1 (part1 input)
      :part2 (part2 input)})

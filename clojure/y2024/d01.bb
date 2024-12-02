#!/usr/bin/env bb

(ns y2024.d01
  (:require [babashka.cli :as cli]))


(def cli-options {:input {:default "../../inputs/2024/01.input" :type :string}})

(def opts (cli/parse-opts *command-line-args* {:spec cli-options}))

(def input
  (->> (slurp (get opts :input))
       (re-seq #"\d+")
       (map parse-long)
       (partition 2)
       (apply map vector)))

(defn part1 [[lhs rhs]]
  (->> (map - (sort lhs) (sort rhs))
       (map abs)
       (reduce +)))

(defn part2 [[lhs rhs]]
  (let [freqs (frequencies rhs)]
    (->> (map #(* % (freqs % 0)) lhs)
         (reduce +))))

(prn {:part1 (part1 input)
      :part2 (part2 input)})

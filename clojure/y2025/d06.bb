#!/usr/bin/env bb

(ns y2025.d06
  (:require [clojure.string :as str]))

(defn parse1 [data]
  (let [rows     (->> data str/split-lines)
        ops      (mapv {"+" + "*" *} (-> rows last (str/split #" +")))
        row-nums (eduction
                  (comp (map str/trim)
                        (map #(str/split % #" +"))
                        (map #(mapv parse-long %)))
                  (pop rows))
        nums     (->> row-nums (into []) (apply mapv vector))]
    [ops nums]))

(defn parse2 [data]
  (let [rows (->> data str/split-lines)
        ops  (->> (str/split (->> rows last) #" +") (mapv {"+" + "*" *}))
        cols (->> rows
                  pop
                  (apply mapv vector)
                  (mapv #(apply str %)) (mapv str/trim)
                  (partition-by str/blank?)
                  (filterv #(-> % first (= "") not))
                  (mapv #(mapv parse-long %)))]
    [ops cols]))

(defn solve [[ops cols]]
  (apply + (mapv #(reduce %1 %2) ops cols)))

(defn part1 [data] (-> data parse1 solve))

(defn part2 [data] (-> data parse2 solve))

(let [input (-> *command-line-args* last slurp)]
  (prn {:part1 (part1 input)
        :part2 (part2 input)}))
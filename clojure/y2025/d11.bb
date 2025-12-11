#!/usr/bin/env bb

 (ns y2025.d11
   (:require [clojure.string :as str]))

(defn parse [s]
  (loop [[line & lines] (str/split-lines s)
         dag {}]
    (if (nil? line)
      dag
      (let [[src & dsts] (str/split line #" ")
            src (subs src 0 (dec (count src)))]
        (recur lines (assoc dag src (set dsts)))))))

(def ways
  (memoize
   (fn [dag src dst]
     (if (= src dst)
       1
       (reduce + (map #(ways dag % dst) (get dag src [])))))))

(defn solve [dag]
  {:part1 (ways dag "you" "out")
   :part2 (->> [["svr" "fft" "dac" "out"]
                ["svr" "dac" "fft" "out"]]
               (map #(->> (partition 2 1 %)
                          (map (fn [[src dst]] (ways dag src dst)))
                          (reduce *)))
               (apply +))})

(when (= *file* (System/getProperty "babashka.file"))
  (let [input (->> *command-line-args* last slurp parse)
        res (solve input)]
    (prn res)))


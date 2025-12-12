#!/usr/bin/env bb

(ns y2025.d12
  (:require [clojure.string :as str]))

(defn parse-region [line]
  (let [[size & cnts] (str/split line #" ")
        [w h] (->> (str/split (subs size 0 (dec (count size))) #"x")
                   (mapv parse-long))]
    [w h (mapv parse-long cnts)]))

(defn parse [s]
  (let [[shapes regions] ((juxt butlast last) (str/split s #"\n\n"))]
    {:shapes (->> shapes (mapv #(frequencies (subs % 2))))
     :regions (mapv parse-region (str/split-lines regions))}))

(defn can-fit? [shapes [w h cnts]]
  (let [pieces (reduce + cnts)
        pieces-pixels (->> (map (fn [shape cnt] (* cnt (shape \# 0))) shapes cnts)
                           (reduce +))
        grid-slots (* (quot w 3) (quot h 3))]
    (cond
      (<= pieces grid-slots)    true  ;; Can be trivially solved by placing the pieces in a grid.
      (> pieces-pixels (* w h)) false ;; Obviously not possible.
      :else                     (throw (ex-info "Dammit Eric." {})))))

(defn solve [{:keys [shapes regions]}]
  {:part1 (count (filter #(can-fit? shapes %) regions))})

(when (= *file* (System/getProperty "babashka.file"))
  (->> *command-line-args* last slurp parse solve prn))

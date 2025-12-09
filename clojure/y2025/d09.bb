#!/usr/bin/env bb

 (ns y2025.d09
   (:require [clojure.string :as str]))


(defn parse [s]
  (->> (str/split-lines s)
       (mapv #(mapv parse-long (str/split % #",")))))

(defn cartesian [x]
  (for [[i xi] (map-indexed vector x)
        yi (subvec x (inc i))]
    [xi yi]))

(defn part1 [tiles]
  (->>
   (for [[[x1 y1] [x2 y2]] (cartesian tiles)]
     (* (->> (- x1 x2) abs (+ 1))
        (->> (- y1 y2) abs (+ 1))))
   (reduce max)))


(defn intersect [[min-x min-y max-x max-y] edges]
  (some (fn [[e-min-x e-min-y e-max-x e-max-y]]
          (and  (< min-x e-max-x) (> max-x e-min-x)
                (< min-y e-max-y) (> max-y e-min-y)))
        edges))

(defn part2 [tiles]
  (let [edges
        (->> (partition 2 1 (conj tiles (tiles 0)))
             (map (fn [[[x1 y1] [x2 y2]]] [(min x1 x2) (min y1 y2) (max x1 x2) (max y1 y2)]))
             (sort-by (fn [[x1 y1 x2 y2]]
                        (+ (- x2 x1) (- y2 y1))) >))]
    (loop [best 0
           [[[xi yi] [xj yj]] & ps :as pairs] (cartesian tiles)]
      (if (empty? pairs)
        best
        (let [min-x (min xi xj)
              max-x (max xi xj)
              min-y (min yi yj)
              max-y (max yi yj)
              area (* (-> 1 (+ max-x) (- min-x))
                      (-> 1 (+ max-y) (- min-y)))]
          (if (and (> area best) (not (intersect [min-x min-y max-x max-y] edges)))
            (recur (max best area) ps)
            (recur best ps)))))))

(when (= *file* (System/getProperty "babashka.file"))
  (let [input (->> *command-line-args* last slurp parse)
        res {:part1 (part1 input)
             :part2 (part2 input)}]
    (prn res)))


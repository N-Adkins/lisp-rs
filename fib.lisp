(def! fibonacci 
      (fn* (a) 
           (if (<= a 1) a 
             (+ (fibonacci (- a 2)) 
                (fibonacci (- a 1))))))

(prn (fibonacci 10))


class VolumeUnit(object):

    def __init__(self, label, amount_in_ml):
        """
        :param label: name of unit (e.g. 'ml', 'g', etc)
        :param amount_in_ml: conversion amount of this unit in ml
        """
        self.__label = label
        self.__amount_in_ml = amount_in_ml
    
    @property
    def label(self):
        return self.__label
     
    def to_milliters(self):
        return round(self.__amount_in_ml, 2)
   
    def to_teaspoons(self):
        return round(self.__amount_in_ml / 4.93, 2)

    def to_tablespoons(self):
        return round(self.__amount_in_ml / 14.79, 2)

    def to_fluid_ounces(self):
        return round(self.__amount_in_ml / 29.57, 2)

    def to_cups(self):
        return round(self.__amount_in_ml / 236.59, 2)

    def to_liters(self):
        return round(self.__amount_in_ml / 1000.00, 2)



class MassUnit(object):

    def __init__(self, label, amount_in_g):
        """
        :param label: name of unit (e.g. 'ml', 'g', etc)
        :param amount_in_g: conversion amount of this unit in grams
        """
        self.__label = label
        self.__amount_in_g = amount_in_g
    
    @property
    def label(self):
        return self.__label

    def to_grams(self):
        return round(self.__amount_in_g, 2)

    def to_ounces(self):
        return round(self.__amount_in_g/ 28.35, 2)

    def to_pounds(self):
        return round(self.__amount_in_g/ 453.59, 2)


# e.g. 1 banana, 1 leaf, etc.
class AmountUnit(object):

    def __init__(self, label):
        """
        :param label: name of unit (e.g. leaf, banana, etc.)
        """
        self.__label = label

    @property
    def label(self):
        return self.__label
 

class Unit:
    MILLILITER = VolumeUnit("ml", 1.00)
    TEASPOON = VolumeUnit("tsp", 4.93)
    TABLSEPOON = VolumeUnit("Tbsp", 14.79)
    FLUID_OUNCE = VolumeUnit("fl oz", 29.57)
    CUP = VolumeUnit("cup", 236.59)
    LITER = VolumeUnit("l", 1000.00)
    GRAM = MassUnit("g", 1.00)
    OUNCE = MassUnit("oz", 28.35)
    POUND = MassUnit("lb", 453.59)

    @classmethod
    def get_unit(cls, label):
        """Return Unit object based on name."""
        if label.lower() == "ml":
            return cls.MILLILITER
        elif label.lower() == "tsp":
            return cls.TEASPOON
        elif label.lower() == "tbsp":
            return cls.TABLSEPOON
        elif label.lower() == "fl oz":
            return cls.FLUID_OUNCE
        elif label.lower() == "cup":
            return cls.CUP
        elif label.lower() == "l":
            return cls.LITER
        elif label.lower() == "g":
            return cls.GRAM
        elif label.lower() == "oz":
            return cls.OUNCE
        elif label.lower() == "lb":
            return cls.POUND
        
        elif label.lower() in ("leaf", "tortilla", "clove", ):
            return AmountUnit(label.lower())
        
        raise ValueError(f"Unknown unit: '{label}'")


if __name__ == "__main__":
    assert VolumeUnit("tsp", 4.93).to_milliters() == 4.93
    assert VolumeUnit("tsp", 4.93).to_tablespoons() == 0.33
    assert VolumeUnit("tsp", 4.93).to_fluid_ounces() == 0.17
    assert VolumeUnit("tsp", 4.93).to_cups() == 0.02




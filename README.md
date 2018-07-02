# CteEPBD

Library implementation and CLI of the ISO EN 52000-1 "Energy performance of buildings" standard to explore NZEB indicators.

Programa de cálculo de la eficiencia energética de los edificios para su aplicación al CTE DB-HE (procedimiento EN ISO 52000-1) y formatos de datos

## Introducción

Este programa, `CteEPBD`, implementa la metodología de cálculo de la eficiencia energética de los edificios descrita en la norma EN ISO 52000-1:2017 de *Eficiencia energética de los edificios. Evaluación global. Parte 1: Marco general y procedimientos* dentro del alcance de la *Directiva 2010/31/UE* relativa a la eficiencia energética de los edificios (EPDB) y del *Documento Básico de Ahorro de Energía* (*DB-HE*) del *Código Técnico de la Edificación* (*CTE*).

El programa calcula la energía suministrada al edificio (desde redes de abastecimiento o producida *in situ*) y la energía exportada (a la red y a usos no EPB) para obtener diversos indicadores de la eficiencia energética del edificio, expresada como energía ponderada (p.e. consumo de energía primaria no renovable, consumo de energía primaria total o fracción renovable del consumo de energía primaria). Para ello, toma en consideración los factores de paso de los distintos vectores energéticos y el factor de exportación (*k_exp*).

## Uso

El programa es autodocumentado y puede obtenerse ayuda usando la opción `-h`.

Una llamada típica al programa:

```$ cteepbd -c test_data/cte_test_carriers.csv -l PENINSULA```

Produce los siguientes resultados por pantalla:

```language-plain

    ** Datos de entrada
    Componentes energéticos: "test_data/cte_test_carriers.csv"
    Factores de paso (archivo): "test_data/factores_paso_test.csv"
    Área de referencia (metadatos) [m2]: 200.00
    Factor de exportación (metadatos) [-]: 0.0
    ** Balance energético
    Area_ref = 200.00 [m2]
    k_exp = 0.00
    C_ep [kWh/m2.an]: ren = 25.4, nren = 19.4, tot = 44.8, RER = 0.57

```

Donde se puede apreciar el resultado del cálculo del consumo de energía primaria renovable (C_ep_ren), no renovable (C_ep_nren) y total (C_ep_tot), además de la fracción renovable (RER).

## Hipótesis de cálculo

Se han adoptado las siguientes hipótesis de cálculo:

- los factores de paso son constantes a lo largo de los pasos de cálculo
- no se definen prioridades para la generación energética (f_we_el_stepA promedio)
- se considera como suministrada toda la energía producida por fuentes distintas a la cogeneración
- el factor de coincidencia de cargas se fija igual a 1.0

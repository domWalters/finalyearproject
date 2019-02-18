# -*- coding: utf-8 -*-
import os
import intrinio
import pandas as pd

intrinio.client.username = 'd26718d74d93a71a12da6e0dec25ee66'   # premium account
intrinio.client.password = '418bc2bbe443bca6f4d866d928daad41'

# Started at 10k

########## Retrieve all symbols#######################
# allcompanies = intrinio.companies()
# list(allcompanies)
# allcompanies.to_csv(path_or_buf="Intrinio_all_symbols.csv", encoding='utf-8')

file_to_open = os.path.join(os.getcwd(), "test-data/Intrinio_all_symbols.csv")
allcompanies = pd.read_csv(file_to_open, encoding='utf-8')

data_folder = os.path.join(os.getcwd(), "test-data/PythonData")
for i in range(13331,15000):
    ticker = allcompanies['ticker'][i]
    try:
        file_date = allcompanies['latest_filing_date'][i][0:4]
    except TypeError as e:
        print('%s (%s) Ignored - No filing data.' % (ticker, i))
        continue

    if file_date == '2018':
        # Calcs
        try:
            fudamentals_calculations = intrinio.financials(ticker, type='QTR', statement='calculations')
            if fudamentals_calculations is not None:
                filename = ticker + '_fudamentals_calculations.csv'
                file_to_open = os.path.join(data_folder, filename)
                fudamentals_calculations.to_csv(file_to_open, encoding='utf-8')
            else :
                print('%s (%s) Ignored - Doesnt have calcs.' % (ticker, i))
                continue
        except (AttributeError, ValueError) as e:
            print('%s (%s) Ignored - Doesnt have calcs.' % (ticker, i))
            continue
        # Price
        try:
            price = intrinio.prices(ticker, frequency='quarterly')
            if price.empty == False:
                filename = ticker + '_price.csv'
                file_to_open = os.path.join(data_folder, filename)
                price.to_csv(file_to_open, encoding='utf-8')
            else :
                print('%s (%s) Ignored - Doesnt have price.' % (ticker, i))
                continue
        except (AttributeError, ValueError) as e:
            print('%s (%s) Ignored - Doesnt have price.' % (ticker, i))
            continue
        # Cash
        try:
            fudamentals_caseflow = intrinio.financials(ticker, type='QTR', statement='cash_flow_statement')
            if fudamentals_caseflow is not None:
                filename = ticker + '_fudamentals_caseflow.csv'
                file_to_open = os.path.join(data_folder, filename)
                fudamentals_caseflow.to_csv(file_to_open, encoding='utf-8')
            else :
                print('%s (%s) Ignored - Doesnt have cash.' % (ticker, i))
                continue
        except (AttributeError, ValueError) as e:
            print('%s (%s) Ignored - Doesnt have cash.' % (ticker, i))
            continue
        # Bal
        try:
            fudamentals_balance = intrinio.financials(ticker, type='QTR', statement='balance_sheet')
            filename = ticker + '_fudamentals_balance.csv'
            if fudamentals_balance is not None:
                filename = ticker + '_fudamentals_balance.csv'
                file_to_open = os.path.join(data_folder, filename)
                fudamentals_balance.to_csv(file_to_open, encoding='utf-8')
            else :
                print('%s (%s) Ignored - Doesnt have bal.' % (ticker, i))
                continue
        except (AttributeError, ValueError) as e:
            print('%s (%s) Ignored - Doesnt have bal.' % (ticker, i))
            continue

        print('%s (%s) Successful.' % (ticker, i))
        continue
    else:
        print('%s (%s) Ignored - Didnt file in 2018.' % (ticker, i))
        continue

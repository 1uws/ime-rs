// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//
// Copyright (c) Microsoft Corporation. All rights reserved

#pragma once

#include "BaseDictionaryEngine.h"

class CTableDictionaryEngine : public CBaseDictionaryEngine
{
public:
    CTableDictionaryEngine(LCID locale, _In_ CFile *pDictionaryFile) : CBaseDictionaryEngine(locale, pDictionaryFile) { }
    virtual ~CTableDictionaryEngine() { }

    // Collect word from phrase string.
    // param
    //     [in] psrgKeyCode - Specified key code pointer
    //     [out] pasrgWordString - Specified returns pointer of word as CStringRange.
    // returns
    //     none.
    VOID CollectWord(const CStringRangeSmart& keyCode, _Inout_ CSampleImeArray<CCandidateListItem> *pItemList);

    VOID CollectWordForWildcard(const CStringRangeSmart& keyCode, _Inout_ CSampleImeArray<CCandidateListItem> *pItemList);

    VOID CollectWordFromConvertedStringForWildcard(const CStringRangeSmart& string, _Inout_ CSampleImeArray<CCandidateListItem> *pItemList);
};

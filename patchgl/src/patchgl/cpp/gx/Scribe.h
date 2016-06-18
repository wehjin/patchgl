//
// Created by Jeffrey Yu on 6/18/16.
//

#ifndef PATCHGL_SCRIBE_H
#define PATCHGL_SCRIBE_H

class Scribe {

public:
    Scribe();

    int getWidth() const;
    int getHeight() const;
    void * getImage() const;
};


#endif //PATCHGL_SCRIBE_H

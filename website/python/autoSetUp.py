import os
import json

class website:
    def __init__(self, title, aboutText, mainColor, secondaryColor, accentColor, backgroundColor) -> None:
        self.title = title
        self.aboutText = aboutText
        self.mainColor = mainColor
        self.secondaryColor = secondaryColor
        self.accentColor = accentColor
        self.backgroundColor = backgroundColor
        
    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, indent=4)


class project: 
    def __init__(self, text, foto) -> None:
        self.text = text
        self.foto = foto
        
    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, indent=4)

        # for arg in args:
        #     self.arr.append[arg]
            
# class contact:  
#     def __init__(self, facebook, linkedin, pinterest, twitter, whatsapp, youtube, instagramm, github) -> None:
#         self.facebook = facebook
#         self.linkedin = linkedin
#         self.pinterest = pinterest
#         self.twitter = twitter
#         self.whatsapp = whatsapp
#         self.youtube = youtube
#         self.instagramm = instagramm
#         self.github = github
            

# Greeting
# Display a title bar.
print("**************************************************")
print("***  Application to automate Portfoliowebsites ***")
print("**************************************************")


print("\n\n")

title = input('\nEnter the title\n') 

aboutText = input('\nEnter the about text\n')

mainColor = input('\nEnter the main color\n')

secondaryColor = input('\nEnter the secondary color\n')

accentColor = input('\nEnter the accent color\n')

backgroundColor = input('\nEnter the background color\n')


protoWebsite = website(title, aboutText, mainColor, secondaryColor, accentColor, backgroundColor)

print(protoWebsite.toJSON())



print("\n\n")
print("Add Projects to your Website \n")

bool = "Y"

projectArr = []


def testForN():
    if (bool != "Y" or bool != "y"):
        bool = input("Press Y to add Projects\n\n")
        if(bool != "N" or bool != "n"):
            bool = "N"
        else:
            testForN()         

while(bool == "Y" or bool == "y"):
    _project = input("Add a text for your project\n")
    _fotoUrl = input("Add a photoURL for your project\n")
    listItemProject = project(_project, _fotoUrl)
    
    projectArr.append(listItemProject.toJSON())
    print(listItemProject.toJSON())
    bool = input("Press Y to add Projects\n\n")
    testForN()
    
while(bool == "Y" or bool == "y"):
    socialMediaDict = dict()
    
    _boolFacebook = input("Press Y to add facebook to your social media\n")
    if(_boolFacebook):
        _linkFacebook =  input("Enter the link for your social media\n")
        socialMediaDict['facebook'] = _linkFacebook
        
    _boollinkedin = input("Press Y to add linkedin toy our social media\n")
    if(_boollinkedin):
        _linklinkedin =  input("Enter the link for your social media\n")
        socialMediaDict['linkedin'] = _linklinkedin
    
    _boolpinterest = input("Press Y to add pinterest toy our social media\n")
    if(_boolpinterest):
        _linkpinterest =  input("Enter the link for your social media\n")
        socialMediaDict['pinterest'] = _linkpinterest
        
    _booltwitter = input("Press Y to add twitter toy our social media\n")
    if(_booltwitter):
        _linktwitter =  input("Enter the link for your social media\n")
        socialMediaDict['twitter'] = _linktwitter
        
    boolwhatsapp = input("Press Y to add whatsapp toy our social media\n")
    if(boolwhatsapp):
        _linkwhatsapp =  input("Enter the link for your social media\n")
        socialMediaDict['whatsapp'] = _linkwhatsapp
        
    boolyoutube= input("Press Y to add youtube toy our social media\n")
    if(boolyoutube):
        _linkyoutube =  input("Enter the link for your social media\n")
        socialMediaDict['youtube'] = _linkyoutube
        
    boolinstagramm= input("Press Y to add instagramm toy our social media\n")
    if(boolinstagramm):
        _linkinstagramm =  input("Enter the link for your social media\n")
        socialMediaDict['instagramm'] = _linkinstagramm
        
    boolgithub = input("Press Y to add github toy our social media\n")
    if(boolgithub):
        _linkgithub =  input("Enter the link for your social media\n")
        socialMediaDict['github'] = _linkgithub
   
   
with open("data.json", "w") as dataJson:
    dataJson.write(protoWebsite.toJSON())
    dataJson.write(json.dumps(projectArr, default=lambda o: o.__dict__, indent=4))
    dataJson.write(json.dumps(socialMediaDict, default=lambda o: o.__dict__, indent=4))

input('')
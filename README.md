# display_off
Powers off all DDC/CI capable displays on Windows shutdown.

# Installation
1) Download `display_off.exe` binary from Releases
2) Move the binary to any folder, i.e. `C:\Program Files\display_off`
3) Execute once for a test. *All the monitors should shut off.*
4) Open Task Scheduler, create an event with a **custom event trigger**.
After creating a task, go *Triggers > New > Custom > Edit Event > XML* and paste the following **(mind the NOTICE below)**:
```xml
<QueryList>
  <Query Id="0" Path="System">
    <Select Path="System">
    *[System[Provider[@Name='User32'] and (Level=4 or Level=0) and (EventID=1074)]]
   and 
     *[EventData[Data[@Name='param5'] and (Data='power off')]]
    </Select>
  </Query>
</QueryList>
```
5) Enable `Run with highest privileges` and save the task

# Notice for non-English Windows locales
`power off` text in the XML will be different for each Windows locale. In order to find out the exact text:
1) Open Event Viewer, then *Windows Logs > System*
2) Filter for EventID `1074`
3) Find a relevant event (RuntimeBroker.exe, 0x0), then in *Details tab -> XML View*
4) Copy the **exact** text from `param5` and paste in XMP above

## Credits
- @manvir-singh for writing an initial code
- [Ste](https://superuser.com/a/1622363) from Superuser forum for an XML trigger example